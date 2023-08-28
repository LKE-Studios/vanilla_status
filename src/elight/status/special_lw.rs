use {
    crate::imports::status_imports::*,
    crate::element::status::special_lw::*
};

#[status_script(agent = "elight", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn elight_special_lw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    element_special_lw_main(fighter)
}

#[status_script(agent = "elight", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn elight_special_lw_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    element_special_lw_end_main(fighter)
}

pub fn install() {
    install_status_scripts!(
        elight_special_lw_main, elight_special_lw_end
    );
}
