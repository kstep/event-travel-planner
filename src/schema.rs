table! {
    event_statuses (id) {
        id -> Integer,
        name -> Text,
    }
}

table! {
    events (id) {
        id -> Integer,
        festival_id -> Integer,
        status -> Integer,
        name -> Nullable<Text>,
        description -> Nullable<Text>,
        place -> Nullable<Text>,
        site_url -> Nullable<Text>,
        facebook_url -> Nullable<Text>,
        register_url -> Nullable<Text>,
        register_start_date -> Nullable<Integer>,
        register_end_date -> Nullable<Integer>,
        start_date -> Integer,
        end_date -> Integer,
    }
}

table! {
    festivals (id) {
        id -> Integer,
        name -> Text,
        description -> Text,
        place -> Nullable<Text>,
        site_url -> Nullable<Text>,
        facebook_url -> Nullable<Text>,
    }
}

joinable!(events -> event_statuses (status));
joinable!(events -> festivals (festival_id));

allow_tables_to_appear_in_same_query!(event_statuses, events, festivals,);
