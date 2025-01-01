#pragma once

namespace core::dataproviders::abc {
    template <typename T, typename Entity>
    class Repository {
    public:
        virtual ~Repository() = default;

        virtual void save(Entity const& entity) = 0;
        virtual void remove(Entity const& entity) = 0;

        virtual Entity const& getById()
    };
}
