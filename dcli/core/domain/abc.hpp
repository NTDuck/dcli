#pragma once

#include <concepts>
#include <type_traits>

namespace core::domain::abc {
    template <std::equality_comparable T>
    class EntityId {};

    template <typename T, typename Id>
    requires std::is_base_of_v<EntityId<T>, Id>
    class Entity {
    public:
        virtual ~Entity() = default;

        virtual Id const& getId() const = 0;

        bool operator==(Entity const& other) const final {
            return getId() == other.getId();
        }

        bool operator!=(Entity const& other) const final = default;
    };
}
