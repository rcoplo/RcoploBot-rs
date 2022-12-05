use crate::domain::table::GroupFunction;
crud!(GroupFunction{});


impl_select!(GroupFunction{select_group_function(group_id:&i64) -> Option  => "`where group_id = #{group_id}`"});
impl_select!(GroupFunction{select_group_function_list()  => "``"});
impl_update!(GroupFunction{update_group_function(group_id:&i64)  => "` where group_id = #{group_id}`"});