use crate::domain::SignGroupUsers;
crud!(SignGroupUsers{});


impl_select!(SignGroupUsers{select_by_user_id_group_id(user_id:&i64,group_id:&i64) -> Option => "`where user_id = #{user_id} and group_id = #{group_id}`"});