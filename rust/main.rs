mod listener;

use clap::Parser;
use dust_dds::domain::domain_participant::DomainParticipant;
use dust_dds::domain::domain_participant_factory::DomainParticipantFactory;
use dust_dds::infrastructure::qos;
use dust_dds::infrastructure::qos_policy::{self as policy, XCDR2_DATA_REPRESENTATION};
use dust_dds::infrastructure::status::StatusKind;
use dust_dds::infrastructure::type_support::DdsType;
use dust_dds::std_runtime::StdRuntime;
use listener::{
    DataReaderListener, DataWriterListener, ParticipantListener, PublisherListener,
    SubscriberListener, TopicListener,
};

#[derive(Clone, Debug, Default, DdsType)]
#[dust_dds(extensibility = "final")]
struct Hello {
    pub participant_name: String,
    pub participant_pretty_version: String,

    pub schema_major: u32,
    pub schema_minor: u32,
    pub schema_patch: u32,
}

const ALL_STATUSES: &[StatusKind] = &[
    StatusKind::InconsistentTopic,
    StatusKind::OfferedDeadlineMissed,
    StatusKind::RequestedDeadlineMissed,
    StatusKind::OfferedIncompatibleQos,
    StatusKind::RequestedIncompatibleQos,
    StatusKind::SampleLost,
    StatusKind::SampleRejected,
    StatusKind::DataOnReaders,
    StatusKind::DataAvailable,
    StatusKind::LivelinessLost,
    StatusKind::LivelinessChanged,
    StatusKind::PublicationMatched,
    StatusKind::SubscriptionMatched,
];

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value_t = tracing::Level::DEBUG)]
    log_level: tracing::Level,

    #[arg(short, long)]
    partition: Option<String>,

    #[arg(short, long, default_value = "hello-rs")]
    name: String,
}

fn get_participant(partition: &[String]) -> DomainParticipant<StdRuntime> {
    let domain = 0;
    let participant = DomainParticipantFactory::get_instance()
        .create_participant(
            domain,
            qos::QosKind::Default,
            Some(ParticipantListener),
            ALL_STATUSES,
        )
        .unwrap();
    participant
        .set_default_subscriber_qos(qos::QosKind::Specific(qos::SubscriberQos {
            partition: policy::PartitionQosPolicy {
                name: partition.to_vec(),
            },
            ..Default::default()
        }))
        .unwrap();
    participant
        .set_default_publisher_qos(qos::QosKind::Specific(qos::PublisherQos {
            partition: policy::PartitionQosPolicy {
                name: partition.to_vec(),
            },
            ..Default::default()
        }))
        .unwrap();
    participant
        .set_default_topic_qos(qos::QosKind::Specific(qos::TopicQos {
            representation: policy::DataRepresentationQosPolicy {
                value: vec![XCDR2_DATA_REPRESENTATION],
            },
            ..Default::default()
        }))
        .unwrap();

    participant
}

fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    let args = Args::parse();
    let partition = if let Some(partition) = &args.partition {
        vec![partition.clone()]
    } else {
        Vec::new() // using "" as the partition seems to break things??
    };

    let filter = tracing_subscriber::EnvFilter::builder()
        // The only logs dust_dds provides is #[tracing::instrument] wrappers around most
        // functions. That's very spammy, and not very helpful for troubleshooting. To get the most
        // value out of logs, implement the various Listener objects.
        .parse(format!("{},dust_dds=INFO", args.log_level))
        .unwrap();
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_writer(std::io::stderr)
        .init();

    let participant = get_participant(&partition);
    let subscriber = participant
        .create_subscriber(
            qos::QosKind::Default,
            Some(SubscriberListener),
            ALL_STATUSES,
        )
        .unwrap();
    let publisher = participant
        .create_publisher(qos::QosKind::Default, Some(PublisherListener), ALL_STATUSES)
        .unwrap();

    let topic = participant
        .create_topic::<Hello>(
            "connection::Hello",
            "connection::Hello",
            qos::QosKind::Default,
            Some(TopicListener),
            ALL_STATUSES,
        )
        .unwrap();

    let representation = policy::DataRepresentationQosPolicy {
        value: vec![XCDR2_DATA_REPRESENTATION],
    };
    let reader = subscriber
        .create_datareader::<Hello>(
            &topic,
            qos::QosKind::Specific(qos::DataReaderQos {
                durability: policy::DurabilityQosPolicy {
                    kind: policy::DurabilityQosPolicyKind::TransientLocal,
                },
                history: policy::HistoryQosPolicy {
                    kind: policy::HistoryQosPolicyKind::KeepAll,
                },
                reliability: policy::ReliabilityQosPolicy {
                    kind: policy::ReliabilityQosPolicyKind::Reliable,
                    max_blocking_time: dust_dds::infrastructure::time::DurationKind::Infinite,
                },
                representation: representation.clone(),
                ..Default::default()
            }),
            Some(DataReaderListener),
            ALL_STATUSES,
        )
        .unwrap();
    let writer = publisher
        .create_datawriter::<Hello>(
            &topic,
            qos::QosKind::Specific(qos::DataWriterQos {
                durability: policy::DurabilityQosPolicy {
                    kind: policy::DurabilityQosPolicyKind::TransientLocal,
                },
                history: policy::HistoryQosPolicy {
                    kind: policy::HistoryQosPolicyKind::KeepAll,
                },
                reliability: policy::ReliabilityQosPolicy {
                    kind: policy::ReliabilityQosPolicyKind::Reliable,
                    max_blocking_time: dust_dds::infrastructure::time::DurationKind::Infinite,
                },
                representation,
                ..Default::default()
            }),
            Some(DataWriterListener),
            ALL_STATUSES,
        )
        .unwrap();

    let own_hello = Hello {
        participant_name: args.name.clone(),
        participant_pretty_version: "hello-rs v0.1.0".to_string(),
        schema_major: 0,
        schema_minor: 1,
        schema_patch: 0,
    };
    tracing::info!("Self: {own_hello:?}");

    writer.write(own_hello, None).unwrap();

    loop {
        std::thread::sleep(std::time::Duration::from_millis(100));
        let Ok(sample) = reader.read_next_sample() else {
            continue;
        };
        let hello = sample.data.unwrap();
        if hello.participant_name == args.name {
            tracing::debug!("Received own hello, ignoring ...");
        } else {
            tracing::info!("Received: {hello:?}");
        }
    }
}
