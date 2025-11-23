/**
    dywoq 2025 - Apache License 2.0

    utility/tuple.h:
        Contains simple std::tuple implementation
*/
#pragma once

#include "../types.h"

namespace dywoq::os::kernel::utility
{
    template <typename... Types> struct tuple;

    template <> struct tuple<>
    {};

    /**
        Allows you to return multiple types
        through functions.
        Simple implementation of std::tuple.
    */
    template <typename Head, typename... Tail>
    struct tuple<Head, Tail...> : tuple<Tail...>
    {
        Head first;

        constexpr tuple() noexcept = default;

        constexpr tuple(Head head, Tail... tail) : tuple<Tail...>(tail...), first(head)
        {}

        constexpr size_t
        max_size() const noexcept
        {
            return 1 + sizeof...(Tail);
        }
    };

    /**
        Tuple-element access helper.
    */
    template <size_t Index, typename Tuple> struct tuple_element;

    template <typename Head, typename... Tail>
    struct tuple_element<0, tuple<Head, Tail...>>
    {
        using type = Head;

        constexpr static Head &
        get(tuple<Head, Tail...> &t)
        {
            return t.first;
        }

        constexpr static const Head &
        get(const tuple<Head, Tail...> &t)
        {
            return t.first;
        }
    };

    template <size_t Index, typename Head, typename... Tail>
    struct tuple_element<Index, tuple<Head, Tail...>>
    {
        using type = typename tuple_element<Index - 1, tuple<Tail...>>::type;

        constexpr static type &
        get(tuple<Head, Tail...> &t)
        {
            tuple<Tail...> &tail = t;
            return tuple_element<Index - 1, tuple<Tail...>>::get(tail);
        }

        constexpr static const type &
        get(const tuple<Head, Tail...> &t)
        {
            const tuple<Tail...> &tail = t;
            return tuple_element<Index - 1, tuple<Tail...>>::get(tail);
        }
    };

    /**
        Gets the tuple element at specified index.
    */
    template <size_t Index, typename... Types>
    constexpr auto &
    get(tuple<Types...> &t)
    {
        return tuple_element<Index, tuple<Types...>>::get(t);
    }

    /**
        Gets the tuple element at specified index.
    */
    template <size_t Index, typename... Types>
    constexpr const auto &
    get(const tuple<Types...> &t)
    {
        return tuple_element<Index, tuple<Types...>>::get(t);
    }
} // namespace dywoq::os::kernel::utility
