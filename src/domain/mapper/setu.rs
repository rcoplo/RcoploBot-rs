use crate::domain::table::Setu;
crud!(Setu{});


impl_select!(Setu{select_by_pid(pid:i64) -> Option =>"`where pid = #{pid}`"});
impl_select!(Setu{select_by_id(id:usize) -> Option =>"`where id = #{id}`"});