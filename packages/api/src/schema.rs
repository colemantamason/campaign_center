// @generated automatically by Diesel CLI.

diesel::table! {
    event_shifts (id) {
        id -> Int4,
        event_id -> Int4,
        start_time -> Timestamptz,
        end_time -> Timestamptz,
        #[max_length = 50]
        timezone -> Varchar,
        capacity -> Nullable<Int4>,
        notes -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    event_signups (id) {
        id -> Int4,
        event_shift_id -> Int4,
        user_id -> Int4,
        #[max_length = 20]
        status -> Varchar,
        notes -> Nullable<Text>,
        signed_up_at -> Timestamptz,
        checked_in_at -> Nullable<Timestamptz>,
        cancelled_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    events (id) {
        id -> Int4,
        organization_id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 50]
        event_type -> Varchar,
        #[max_length = 20]
        visibility -> Varchar,
        description -> Nullable<Text>,
        attendee_message -> Nullable<Text>,
        image_url -> Nullable<Text>,
        location_in_person -> Nullable<Text>,
        location_online -> Nullable<Text>,
        communication_bring_a_friend -> Bool,
        communication_other_events -> Bool,
        communication_confirmation -> Bool,
        communication_check_in -> Bool,
        #[max_length = 255]
        contact_name -> Varchar,
        #[max_length = 255]
        contact_email -> Nullable<Varchar>,
        #[max_length = 20]
        contact_phone -> Nullable<Varchar>,
        co_hosts -> Array<Nullable<Text>>,
        invite_groups -> Array<Nullable<Text>>,
        created_by -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    invitations (id) {
        id -> Int4,
        organization_id -> Int4,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 50]
        role -> Varchar,
        token -> Uuid,
        #[max_length = 50]
        status -> Varchar,
        invited_by -> Int4,
        created_at -> Timestamptz,
        expires_at -> Timestamptz,
        accepted_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    notifications (id) {
        id -> Int4,
        user_id -> Int4,
        organization_id -> Nullable<Int4>,
        #[max_length = 50]
        notification_type -> Varchar,
        #[max_length = 255]
        title -> Varchar,
        message -> Text,
        link -> Nullable<Text>,
        read -> Bool,
        read_at -> Nullable<Timestamptz>,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    organization_members (id) {
        id -> Int4,
        organization_id -> Int4,
        user_id -> Int4,
        #[max_length = 50]
        role -> Varchar,
        invited_by -> Nullable<Int4>,
        joined_at -> Timestamptz,
        last_active_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    organizations (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 100]
        slug -> Varchar,
        #[max_length = 50]
        organization_type -> Varchar,
        description -> Nullable<Text>,
        avatar_url -> Nullable<Text>,
        website_url -> Nullable<Text>,
        #[max_length = 255]
        email -> Nullable<Varchar>,
        #[max_length = 20]
        phone_number -> Nullable<Varchar>,
        #[max_length = 255]
        address_line_1 -> Nullable<Varchar>,
        #[max_length = 255]
        address_line_2 -> Nullable<Varchar>,
        #[max_length = 100]
        city -> Nullable<Varchar>,
        #[max_length = 50]
        state -> Nullable<Varchar>,
        #[max_length = 20]
        zip_code -> Nullable<Varchar>,
        #[max_length = 2]
        country -> Nullable<Varchar>,
        #[max_length = 50]
        timezone -> Varchar,
        subscriptions -> Array<Nullable<Text>>,
        created_by -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    password_reset_tokens (id) {
        id -> Int4,
        user_id -> Int4,
        token -> Uuid,
        created_at -> Timestamptz,
        expires_at -> Timestamptz,
        used_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    sessions (id) {
        id -> Int4,
        token -> Uuid,
        user_id -> Int4,
        active_organization_membership_id -> Nullable<Int4>,
        user_agent -> Nullable<Text>,
        ip_address -> Nullable<Inet>,
        created_at -> Timestamptz,
        expires_at -> Timestamptz,
        last_accessed_at -> Timestamptz,
        #[max_length = 20]
        platform -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 255]
        email -> Varchar,
        email_verified_at -> Nullable<Timestamptz>,
        #[max_length = 255]
        password_hash -> Varchar,
        #[max_length = 100]
        first_name -> Varchar,
        #[max_length = 100]
        last_name -> Varchar,
        #[max_length = 20]
        phone_number -> Nullable<Varchar>,
        phone_number_verified_at -> Nullable<Timestamptz>,
        avatar_url -> Nullable<Text>,
        #[max_length = 50]
        timezone -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        last_login_at -> Nullable<Timestamptz>,
    }
}

diesel::joinable!(event_shifts -> events (event_id));
diesel::joinable!(event_signups -> event_shifts (event_shift_id));
diesel::joinable!(event_signups -> users (user_id));
diesel::joinable!(events -> organizations (organization_id));
diesel::joinable!(events -> users (created_by));
diesel::joinable!(invitations -> organizations (organization_id));
diesel::joinable!(invitations -> users (invited_by));
diesel::joinable!(notifications -> organizations (organization_id));
diesel::joinable!(notifications -> users (user_id));
diesel::joinable!(organization_members -> organizations (organization_id));
diesel::joinable!(organizations -> users (created_by));
diesel::joinable!(password_reset_tokens -> users (user_id));
diesel::joinable!(sessions -> organization_members (active_organization_membership_id));
diesel::joinable!(sessions -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    event_shifts,
    event_signups,
    events,
    invitations,
    notifications,
    organization_members,
    organizations,
    password_reset_tokens,
    sessions,
    users,
);
