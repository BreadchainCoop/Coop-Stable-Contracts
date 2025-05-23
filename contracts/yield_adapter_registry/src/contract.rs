use crate::{
    events::YieldAdapterRegistryEvents,
    storage::{
        get_yield_adapter, 
        get_yield_adapters, 
        get_yield_adapters_with_assets, 
        is_asset_supported,
        register_yield_adapter, 
        remove_asset_support, 
        remove_yield_adapter, 
        support_asset,
        verify_if_yield_adapter_exists,
    },
    error::YieldAdapterRegistryError
};
use access_control::{access::default_access_control, constants::DEFAULT_ADMIN_ROLE};
use soroban_sdk::{contract, contractimpl, contractmeta, panic_with_error, Address, Env, Symbol, Vec};

contractmeta!(
    key = "Description",
    val = "Yield adapter registry for the Coopstable cUSD system"
);
fn only_admin(e: &Env, caller: Address) {
    let access_control = default_access_control(e);
    access_control.only_role(&e, &caller, DEFAULT_ADMIN_ROLE);
}
pub trait YieldAdapterRegistryTrait {
    fn __constructor(e: Env, admin: Address);
    fn set_yield_adapter_admin(e: &Env, caller: Address, new_admin: Address);
    fn register_adapter(
        e: &Env,
        caller: Address,
        yield_type: Symbol,
        protocol: Symbol,
        adapter_address: Address,
    );
    fn get_adapter(e: &Env, yield_type: Symbol, protocol: Symbol) -> Address;
    fn get_adapters(e: &Env, yield_type: Symbol) -> Vec<Address>;
    fn get_adapters_with_assets(e: &Env, yield_type: Symbol) -> Vec<(Address, Vec<Address>)>;
    fn remove_adapter(e: &Env, caller: Address, yield_type: Symbol, protocol: Symbol);
    fn add_support_for_asset(
        e: &Env,
        caller: Address,
        yield_type: Symbol,
        protocol: Symbol,
        asset_address: Address,
    );
    fn remove_support_for_asset(
        e: &Env,
        caller: Address,
        yield_type: Symbol,
        protocol: Symbol,
        asset_address: Address,
    );
    fn is_supported_asset(
        e: &Env,
        yield_type: Symbol,
        protocol: Symbol,
        asset_address: Address,
    ) -> bool;
}

#[contract]
pub struct YieldAdapterRegistry;

#[contractimpl]
impl YieldAdapterRegistryTrait for YieldAdapterRegistry {
    fn __constructor(e: Env, admin: Address) {
        let access_control = default_access_control(&e);

        access_control.initialize(&e, &admin);
        access_control.set_role_admin(&e, DEFAULT_ADMIN_ROLE, DEFAULT_ADMIN_ROLE);
        access_control._grant_role(&e, DEFAULT_ADMIN_ROLE, &admin);
    }

    fn set_yield_adapter_admin(e: &Env, caller: Address, new_admin: Address) {
        let access_control = default_access_control(e);
        access_control.grant_role(&e, caller, DEFAULT_ADMIN_ROLE, &new_admin);
        YieldAdapterRegistryEvents::set_admin(&e, new_admin);
    }

    fn register_adapter(
        e: &Env,
        caller: Address,
        yield_type: Symbol,
        protocol: Symbol,
        adapter_address: Address,
    ) {
        only_admin(e, caller);
        register_yield_adapter(
            e,
            yield_type.clone(),
            protocol.clone(),
            adapter_address.clone(),
        );
        YieldAdapterRegistryEvents::register_adapter(&e, yield_type, protocol, adapter_address);
    }

    fn remove_adapter(e: &Env, caller: Address, yield_type: Symbol, protocol: Symbol) {
        only_admin(e, caller);
        let adapter_address = get_yield_adapter(e, yield_type.clone(), protocol.clone());
        remove_yield_adapter(e, yield_type.clone(), protocol.clone());
        YieldAdapterRegistryEvents::remove_adapter(&e, yield_type, protocol, adapter_address);
    }

    fn get_adapter(e: &Env, yield_type: Symbol, protocol: Symbol) -> Address {
        if !verify_if_yield_adapter_exists(e, yield_type.clone(), protocol.clone()) {
            panic_with_error!(e, YieldAdapterRegistryError::InvalidYieldAdapter);
        }
        get_yield_adapter(e, yield_type, protocol)
    }

    fn add_support_for_asset(
        e: &Env,
        caller: Address,
        yield_type: Symbol,
        protocol: Symbol,
        asset_address: Address,
    ) {
        only_admin(e, caller);
        support_asset(
            e,
            yield_type.clone(),
            protocol.clone(),
            asset_address.clone(),
        );
        YieldAdapterRegistryEvents::add_support_for_asset(&e, yield_type, protocol, asset_address);
    }

    fn remove_support_for_asset(
        e: &Env,
        caller: Address,
        yield_type: Symbol,
        protocol: Symbol,
        asset_address: Address,
    ) {
        only_admin(e, caller);
        remove_asset_support(
            &e,
            yield_type.clone(),
            protocol.clone(),
            asset_address.clone(),
        );
        YieldAdapterRegistryEvents::remove_support_for_asset(
            &e,
            yield_type,
            protocol,
            asset_address,
        );
    }

    fn is_supported_asset(
        e: &Env,
        yield_type: Symbol,
        protocol: Symbol,
        asset_address: Address,
    ) -> bool {
        is_asset_supported(e, yield_type, protocol, asset_address)
    }

    fn get_adapters(e: &Env, yield_type: Symbol) -> Vec<Address> {
        get_yield_adapters(e, yield_type)
    }

    fn get_adapters_with_assets(e: &Env, yield_type: Symbol) -> Vec<(Address, Vec<Address>)> {
        get_yield_adapters_with_assets(e, yield_type)
    }
}
