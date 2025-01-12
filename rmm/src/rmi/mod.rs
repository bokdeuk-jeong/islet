pub mod constraint;
pub mod error;
pub mod features;
pub mod gpt;
pub mod realm;
pub mod rec;
pub mod rtt;
pub mod version;

use crate::define_interface;
use crate::rmi::error::Error;
use crate::rmi::realm::Rd;
use crate::rmi::rec::run::Run;

define_interface! {
    command {
         VERSION                = 0xc400_0150,
         GRANULE_DELEGATE       = 0xc400_0151,
         GRANULE_UNDELEGATE     = 0xc400_0152,
         DATA_CREATE            = 0xc400_0153,
         DATA_CREATE_UNKNOWN    = 0xc400_0154,
         DATA_DESTROY           = 0xc400_0155,
         REALM_ACTIVATE         = 0xc400_0157,
         REALM_CREATE           = 0xc400_0158,
         REALM_DESTROY          = 0xc400_0159,
         REC_CREATE             = 0xc400_015a,
         REC_DESTROY            = 0xc400_015b,
         REC_ENTER              = 0xc400_015c,
         RTT_CREATE             = 0xc400_015d,
         RTT_DESTROY            = 0xc400_015e,
         RTT_MAP_UNPROTECTED    = 0xc400_015f,
         RTT_UNMAP_UNPROTECTED  = 0xc400_0162,
         RTT_READ_ENTRY         = 0xc400_0161,
         FEATURES               = 0xc400_0165,
         REC_AUX_COUNT          = 0xc400_0167,
         RTT_INIT_RIPAS         = 0xc400_0168,
         RTT_SET_RIPAS          = 0xc400_0169,
    }
}

pub const REQ_COMPLETE: usize = 0xc400_018f;

pub const GET_REALM_ATTEST_KEY: usize = 0xC400_01B2;
pub const GET_PLAT_TOKEN: usize = 0xC400_01B3;

pub const BOOT_COMPLETE: usize = 0xC400_01CF;
pub const BOOT_SUCCESS: usize = 0x0;

pub const NOT_SUPPORTED_YET: usize = 0xFFFF_EEEE;

pub const ABI_MAJOR_VERSION: usize = 1;
pub const ABI_MINOR_VERSION: usize = 0;

pub const HASH_ALGO_SHA256: u8 = 0;
pub const HASH_ALGO_SHA512: u8 = 1;

pub const RET_FAIL: usize = 0x100;
pub const RET_EXCEPTION_IRQ: usize = 0x0;
pub const RET_EXCEPTION_SERROR: usize = 0x1;
pub const RET_EXCEPTION_TRAP: usize = 0x2;
pub const RET_EXCEPTION_IL: usize = 0x3;

pub const SUCCESS: usize = 0;
pub const ERROR_INPUT: usize = 1;
pub const ERROR_REC: usize = 3;
pub const SUCCESS_REC_ENTER: usize = 4;

// RmiRttEntryState represents the state of an RTTE
pub mod rtt_entry_state {
    pub const RMI_UNASSIGNED: usize = 0;
    pub const RMI_DESTROYED: usize = 1;
    pub const RMI_ASSIGNED: usize = 2;
    pub const RMI_TABLE: usize = 3;
    pub const RMI_VALID_NS: usize = 4;
}

pub const MAX_REC_AUX_GRANULES: usize = 16;

pub const EXIT_SYNC: u8 = 0;
pub const EXIT_IRQ: u8 = 1;
pub const EXIT_FIQ: u8 = 2;
pub const EXIT_PSCI: u8 = 3;
pub const EXIT_RIPAS_CHANGE: u8 = 4;
pub const EXIT_HOST_CALL: u8 = 5;
pub const EXIT_SERROR: u8 = 6;

pub type RMI = &'static dyn Interface;

pub struct MapProt(usize);

impl From<usize> for MapProt {
    fn from(prot: usize) -> Self {
        Self(prot as usize)
    }
}

impl MapProt {
    pub fn new(data: usize) -> Self {
        MapProt(data)
    }
    pub fn set_bit(&mut self, prot: u64) {
        self.0 |= 1 << prot;
    }
    pub fn get(&self) -> usize {
        self.0
    }
    pub fn is_set(&self, prot: u64) -> bool {
        (self.0 >> prot) & 1 == 1
    }
    pub const DEVICE: u64 = 0;
    pub const NS_PAS: u64 = 1;
}

pub trait Interface {
    // TODO: it would be better to leave only true RMI interface here
    //       while moving others to another place (e.g., set_reg())
    fn create_realm(&self, vmid: u16, rtt_base: usize) -> Result<usize, Error>;
    fn create_vcpu(&self, id: usize) -> Result<usize, Error>;
    fn realm_config(&self, id: usize, config_ipa: usize, ipa_bits: usize) -> Result<(), Error>;
    fn remove(&self, id: usize) -> Result<(), Error>;
    fn run(&self, id: usize, vcpu: usize, incr_pc: usize) -> Result<[usize; 4], Error>;
    fn rtt_create(
        &self,
        id: usize,
        rtt_addr: usize,
        guest: usize,
        level: usize,
    ) -> Result<(), Error>;
    fn rtt_destroy(
        &self,
        rd: &Rd,
        rtt_addr: usize,
        guest: usize,
        level: usize,
    ) -> Result<(), Error>;
    fn rtt_init_ripas(&self, id: usize, guest: usize, level: usize) -> Result<(), Error>;
    fn rtt_get_ripas(&self, id: usize, ipa: usize, level: usize) -> Result<u64, Error>;
    fn rtt_read_entry(&self, id: usize, guest: usize, level: usize) -> Result<[usize; 4], Error>;
    fn rtt_map_unprotected(
        &self,
        rd: &Rd,
        guest: usize,
        level: usize,
        host_s2tte: usize,
    ) -> Result<(), Error>;
    fn rtt_unmap_unprotected(&self, id: usize, guest: usize, level: usize) -> Result<(), Error>;
    fn data_create(&self, id: usize, guest: usize, target_pa: usize) -> Result<(), Error>;
    fn data_destroy(&self, id: usize, guest: usize) -> Result<usize, Error>;
    fn make_shared(&self, id: usize, guest: usize, level: usize) -> Result<(), Error>;
    fn make_exclusive(&self, id: usize, guest: usize, level: usize) -> Result<(), Error>;
    fn set_reg(&self, id: usize, vcpu: usize, register: usize, value: usize) -> Result<(), Error>;
    fn get_reg(&self, id: usize, vcpu: usize, register: usize) -> Result<usize, Error>;
    fn receive_gic_state_from_host(&self, id: usize, vcpu: usize, run: &Run) -> Result<(), Error>;
    fn send_gic_state_to_host(&self, id: usize, vcpu: usize, run: &mut Run) -> Result<(), Error>;
    fn send_timer_state_to_host(&self, id: usize, vcpu: usize, run: &mut Run) -> Result<(), Error>;
    fn emulate_mmio(&self, id: usize, vcpu: usize, run: &Run) -> Result<(), Error>;
}

pub(crate) fn dummy() {
    trace!("Dummy implementation.");
}
