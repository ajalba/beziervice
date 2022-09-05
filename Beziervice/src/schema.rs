// @generated automatically by Diesel CLI.

diesel::table! {
    curves (id) {
        id -> Int4,
        curve_name -> Text,
        expression_x -> Text,
        expression_y -> Text,
        expression_z -> Text,
        points_x -> Array<Nullable<Float8>>,
        points_y -> Array<Nullable<Float8>>,
        points_z -> Array<Nullable<Float8>>,
    }
}
