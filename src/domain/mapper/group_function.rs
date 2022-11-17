use crate::domain::table::*;
crud!(GroupFunction{});


impl_select!(GroupFunction{select_group_function(group_id:i64)  => "`where group_id = #{group_id}`"});