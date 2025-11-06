use dust_dds::dds_async as dds;
use dust_dds::domain::domain_participant_listener::DomainParticipantListener;
use dust_dds::infrastructure::status;
use dust_dds::publication::publisher_listener::PublisherListener as IPublisherListener;
use dust_dds::runtime::DdsRuntime;
use dust_dds::subscription::data_reader_listener::DataReaderListener as IDataReaderListener;
use dust_dds::subscription::subscriber_listener::SubscriberListener as ISubscriberListener;
use dust_dds::topic_definition::topic_listener::TopicListener as ITopicListener;

pub struct ParticipantListener;
impl<R: DdsRuntime> DomainParticipantListener<R> for ParticipantListener {
    async fn on_inconsistent_topic(
        &mut self,
        topic: dds::topic::TopicAsync<R>,
        status: status::InconsistentTopicStatus,
    ) {
        tracing::warn!("Inconsistent topic: {}: {status:?}", topic.get_name());
    }

    async fn on_liveliness_lost(
        &mut self,
        writer: dds::data_writer::DataWriterAsync<R, ()>,
        status: status::LivelinessLostStatus,
    ) {
        tracing::warn!(
            "Liveliness lost: {}: {status:?}",
            writer.get_topic().get_name()
        );
    }

    async fn on_offered_deadline_missed(
        &mut self,
        writer: dds::data_writer::DataWriterAsync<R, ()>,
        _status: status::OfferedDeadlineMissedStatus,
    ) {
        tracing::warn!(
            "Offered deadline missed. topic: {}",
            writer.get_topic().get_name()
        );
    }

    async fn on_offered_incompatible_qos(
        &mut self,
        writer: dds::data_writer::DataWriterAsync<R, ()>,
        _status: status::OfferedIncompatibleQosStatus,
    ) {
        tracing::warn!(
            "Offered incompatible QoS. topic: {}",
            writer.get_topic().get_name()
        );
    }

    async fn on_sample_lost(
        &mut self,
        reader: dds::data_reader::DataReaderAsync<R, ()>,
        _status: status::SampleLostStatus,
    ) {
        tracing::warn!(
            "Sample lost. topic: {}",
            reader.get_topicdescription().get_name()
        );
    }

    async fn on_data_available(&mut self, reader: dds::data_reader::DataReaderAsync<R, ()>) {
        tracing::info!(
            "Data available. topic: {}",
            reader.get_topicdescription().get_name()
        );
    }

    async fn on_sample_rejected(
        &mut self,
        reader: dds::data_reader::DataReaderAsync<R, ()>,
        status: status::SampleRejectedStatus,
    ) {
        tracing::warn!(
            "Sample rejected. topic: {} reason: {:?}",
            reader.get_topicdescription().get_name(),
            status.last_reason
        );
    }

    async fn on_liveliness_changed(
        &mut self,
        reader: dds::data_reader::DataReaderAsync<R, ()>,
        _status: status::LivelinessChangedStatus,
    ) {
        tracing::warn!(
            "Liveliness changed. topic: {}",
            reader.get_topicdescription().get_name()
        );
    }

    async fn on_requested_deadline_missed(
        &mut self,
        reader: dds::data_reader::DataReaderAsync<R, ()>,
        _status: status::RequestedDeadlineMissedStatus,
    ) {
        tracing::warn!(
            "Requested deadline missed. topic: {}",
            reader.get_topicdescription().get_name()
        );
    }

    async fn on_requested_incompatible_qos(
        &mut self,
        reader: dds::data_reader::DataReaderAsync<R, ()>,
        _status: status::RequestedIncompatibleQosStatus,
    ) {
        tracing::warn!(
            "Requested incompatible QoS. topic: {}",
            reader.get_topicdescription().get_name()
        );
    }

    async fn on_publication_matched(
        &mut self,
        writer: dds::data_writer::DataWriterAsync<R, ()>,
        _status: status::PublicationMatchedStatus,
    ) {
        tracing::info!(
            "Publication matched. topic: {}",
            writer.get_topic().get_name()
        );
    }

    async fn on_subscription_matched(
        &mut self,
        reader: dds::data_reader::DataReaderAsync<R, ()>,
        _status: status::SubscriptionMatchedStatus,
    ) {
        tracing::info!(
            "Subscription matched. topic: {}",
            reader.get_topicdescription().get_name()
        );
    }
}

pub struct SubscriberListener;
impl<R: DdsRuntime> ISubscriberListener<R> for SubscriberListener {
    async fn on_data_on_readers(&mut self, subscriber: dds::subscriber::SubscriberAsync<R>) {
        tracing::debug!(
            "Data on readers. subscriber: {:?}",
            subscriber.get_instance_handle().await
        );
    }

    async fn on_data_available(&mut self, reader: dds::data_reader::DataReaderAsync<R, ()>) {
        tracing::info!(
            "Data available. topic: {}",
            reader.get_topicdescription().get_name()
        );
    }

    async fn on_sample_rejected(
        &mut self,
        reader: dds::data_reader::DataReaderAsync<R, ()>,
        status: status::SampleRejectedStatus,
    ) {
        tracing::warn!(
            "Sample rejected. topic: {} reason: {:?}",
            reader.get_topicdescription().get_name(),
            status.last_reason
        );
    }

    async fn on_liveliness_changed(
        &mut self,
        reader: dds::data_reader::DataReaderAsync<R, ()>,
        _status: status::LivelinessChangedStatus,
    ) {
        tracing::warn!(
            "Liveliness changed. topic: {}",
            reader.get_topicdescription().get_name()
        );
    }

    async fn on_requested_deadline_missed(
        &mut self,
        reader: dds::data_reader::DataReaderAsync<R, ()>,
        _status: status::RequestedDeadlineMissedStatus,
    ) {
        tracing::warn!(
            "Requested deadline missed. topic: {}",
            reader.get_topicdescription().get_name()
        );
    }

    async fn on_requested_incompatible_qos(
        &mut self,
        reader: dds::data_reader::DataReaderAsync<R, ()>,
        _status: status::RequestedIncompatibleQosStatus,
    ) {
        tracing::warn!(
            "Requested incompatible QoS. topic: {}",
            reader.get_topicdescription().get_name()
        );
    }

    async fn on_subscription_matched(
        &mut self,
        reader: dds::data_reader::DataReaderAsync<R, ()>,
        _status: status::SubscriptionMatchedStatus,
    ) {
        tracing::info!(
            "Subscription matched. topic: {}",
            reader.get_topicdescription().get_name()
        );
    }

    async fn on_sample_lost(
        &mut self,
        reader: dds::data_reader::DataReaderAsync<R, ()>,
        _status: status::SampleLostStatus,
    ) {
        tracing::warn!(
            "Sample lost. topic: {}",
            reader.get_topicdescription().get_name()
        );
    }
}

pub struct PublisherListener;
impl<R: DdsRuntime> IPublisherListener<R> for PublisherListener {
    async fn on_liveliness_lost(
        &mut self,
        writer: dds::data_writer::DataWriterAsync<R, ()>,
        status: status::LivelinessLostStatus,
    ) {
        tracing::warn!(
            "Liveliness lost: {}: {status:?}",
            writer.get_topic().get_name()
        );
    }

    async fn on_offered_deadline_missed(
        &mut self,
        writer: dds::data_writer::DataWriterAsync<R, ()>,
        _status: status::OfferedDeadlineMissedStatus,
    ) {
        tracing::warn!(
            "Offered deadline missed. topic: {}",
            writer.get_topic().get_name()
        );
    }

    async fn on_offered_incompatible_qos(
        &mut self,
        writer: dds::data_writer::DataWriterAsync<R, ()>,
        _status: status::OfferedIncompatibleQosStatus,
    ) {
        tracing::warn!(
            "Offered incompatible QoS. topic: {}",
            writer.get_topic().get_name()
        );
    }

    async fn on_publication_matched(
        &mut self,
        writer: dds::data_writer::DataWriterAsync<R, ()>,
        _status: status::PublicationMatchedStatus,
    ) {
        tracing::info!(
            "Publication matched. topic: {}",
            writer.get_topic().get_name()
        );
    }
}

pub struct TopicListener;
impl<R: DdsRuntime> ITopicListener<R> for TopicListener {
    async fn on_inconsistent_topic(
        &mut self,
        topic: dds::topic::TopicAsync<R>,
        status: status::InconsistentTopicStatus,
    ) {
        tracing::warn!("Inconsistent topic: {}: {status:?}", topic.get_name());
    }
}

pub struct DataReaderListener;
impl<R: DdsRuntime, DataT: Send> IDataReaderListener<R, DataT> for DataReaderListener {
    async fn on_data_available(&mut self, reader: dds::data_reader::DataReaderAsync<R, DataT>) {
        tracing::info!(
            "Data available. topic: {}",
            reader.get_topicdescription().get_name()
        );
    }

    async fn on_sample_rejected(
        &mut self,
        reader: dds::data_reader::DataReaderAsync<R, DataT>,
        status: status::SampleRejectedStatus,
    ) {
        tracing::warn!(
            "Sample rejected. topic: {} reason: {:?}",
            reader.get_topicdescription().get_name(),
            status.last_reason
        );
    }

    async fn on_liveliness_changed(
        &mut self,
        reader: dds::data_reader::DataReaderAsync<R, DataT>,
        _status: status::LivelinessChangedStatus,
    ) {
        tracing::warn!(
            "Liveliness changed. topic: {}",
            reader.get_topicdescription().get_name()
        );
    }

    async fn on_requested_deadline_missed(
        &mut self,
        reader: dds::data_reader::DataReaderAsync<R, DataT>,
        _status: status::RequestedDeadlineMissedStatus,
    ) {
        tracing::warn!(
            "Requested deadline missed. topic: {}",
            reader.get_topicdescription().get_name()
        );
    }

    async fn on_requested_incompatible_qos(
        &mut self,
        reader: dds::data_reader::DataReaderAsync<R, DataT>,
        _status: status::RequestedIncompatibleQosStatus,
    ) {
        tracing::warn!(
            "Requested incompatible QoS. topic: {}",
            reader.get_topicdescription().get_name()
        );
    }

    async fn on_subscription_matched(
        &mut self,
        reader: dds::data_reader::DataReaderAsync<R, DataT>,
        _status: status::SubscriptionMatchedStatus,
    ) {
        tracing::info!(
            "Subscription matched. topic: {}",
            reader.get_topicdescription().get_name()
        );
    }

    async fn on_sample_lost(
        &mut self,
        reader: dds::data_reader::DataReaderAsync<R, DataT>,
        _status: status::SampleLostStatus,
    ) {
        tracing::warn!(
            "Sample lost. topic: {}",
            reader.get_topicdescription().get_name()
        );
    }
}
