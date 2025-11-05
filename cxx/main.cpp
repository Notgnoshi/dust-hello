#include <cnhos-log/logger.hpp>

#include <field_computer_external_api_idl/connection/Hello.hpp>
#include <field_computer_external_api_idl/connection/constants.hpp>
#include <hurrier/log/logger.hpp>
#include <hurrier/participant.hpp>
#include <hurrier/subscriber_listener.hpp>

const static char* const OWN_NAME = "hello-cpp";
const static cnhos::log::LoggerInitializer G_LOGGER(OWN_NAME);
CNHOS_LOG_CATEGORY(s_hello, OWN_NAME);

void on_sub(const std::string& uid, bool matched)
{
    CNHOS_LOG_DEBUG(s_hello, "Subscriber " << uid << " " << (matched ? "matched" : "unmatched"));
}

void on_pub(const std::string& uid, bool matched)
{
    CNHOS_LOG_DEBUG(s_hello, "Publisher " << uid << " " << (matched ? "matched" : "unmatched"));
}

void on_hello(const connection::Hello& hello)
{
    if (hello.participant_name() == OWN_NAME)
    {
        CNHOS_LOG_DEBUG(s_hello, "Received Hello from self, ignoring ...");
        return;
    }
    CNHOS_LOG_INFO(s_hello,
                   "Received Hello from: " << hello.participant_name()
                                           << " version: " << hello.schema_major() << "."
                                           << hello.schema_minor() << "." << hello.schema_patch());
}

int main()
{
    hurrier::log::Logger::init();
    cnhos::log::logger_set_level(cnhos::log::Log_level::debug);

    const auto own_hello = connection::Hello(OWN_NAME,
                                             "hello-cpp-v0.1.0",
                                             connection::SCHEMA_MAJOR_VERSION,
                                             connection::SCHEMA_MINOR_VERSION,
                                             connection::SCHEMA_PATCH_VERSION);
    CNHOS_LOG_INFO(s_hello,
                   "Self name: " << own_hello.participant_name()
                                 << " version: " << own_hello.schema_major() << "."
                                 << own_hello.schema_minor() << "." << own_hello.schema_patch());

    auto qos = qos_t::default_shared_qos();
    qos->transport_enable_shm(false);
    qos->transport_enable_udp(true);
    // qos->set_partition("");

    auto participant = hurrier::Participant("hello-cpp", qos);
    auto sub_listener =
        std::make_unique<hurrier::SubscriberListener<connection::Hello>>(on_hello, on_sub);
    auto pub_listener = std::make_unique<hurrier::PublisherListener>(on_pub);

    // TODO: Set history depth in QoS
    // TODO: Set liveliness in QoS
    auto publisher =
        participant.publisher<connection::Hello>("connection::Hello", nullptr, pub_listener.get());
    auto subscriber =
        participant.subscriber<connection::Hello>("connection::Hello", sub_listener.get());

    publisher->publish(own_hello);

    while (true)
    {
        std::this_thread::sleep_for(std::chrono::milliseconds(100));
    }

    return 0;
};
