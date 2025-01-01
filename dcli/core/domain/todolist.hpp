#pragma once

#include <cstdint>
#include <string>

#include <dcli/core/domain/abc.hpp>

namespace core::domain {
    class Task : public abc:: {
    public:
        class Id;
        enum class Status;

        std::string description;
        Status status;
    };

    class Task::Id : public abc::EntityId<Task::Id> {
    public:
        std::string uuid;

        bool operator==(Id const& other) const = default;
        bool operator!=(Id const& other) const = default;
    };

    enum class Task::Status {
        PENDING,
        IN_PROGRESS,
        COMPLETED,
    };
}
