use core::time::Duration;

use crate::smccc::*;

pub struct Psci;

impl Psci {
    pub fn version<C: SmcccCall32>() -> Result<PsciVersion, PsciError> {
        let res = C::call(PSCI_VERSION, [0; 7]).map_err(PsciError::from)?;
        Ok(PsciVersion {
            major: ((res[0] >> 16) & 0x7FFF) as usize,
            minor: (res[0] & 0xFFFF) as usize,
        })
    }

    pub fn cpu_suspend_32<C: SmcccCall32>(
        power_state: u32,
        entry_point_addr: u32,
        context_id: u32,
    ) -> Result<(), PsciError> {
        C::call(
            PSCI_CPU_SUSPEND_32,
            [power_state, entry_point_addr, context_id, 0, 0, 0, 0],
        )
        .map_err(PsciError::from)?;

        Ok(())
    }

    pub fn cpu_suspend_64<C: SmcccCall64>(
        power_state: u32,
        entry_point_addr: u64,
        context_id: u64,
    ) -> Result<(), PsciError> {
        C::call(
            PSCI_CPU_SUSPEND_64,
            [
                power_state as u64,
                entry_point_addr,
                context_id,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
        )
        .map_err(PsciError::from)?;

        Ok(())
    }

    pub fn cpu_off<C: SmcccCall32>() -> Result<(), PsciError> {
        C::call(PSCI_CPU_OFF, [0, 0, 0, 0, 0, 0, 0]).map_err(PsciError::from)?;
        Ok(())
    }

    pub fn cpu_on_32<C: SmcccCall32>(
        target_cpu: u32,
        entry_point_addr: u32,
        context_id: u32,
    ) -> Result<(), PsciError> {
        C::call(
            PSCI_CPU_ON_32,
            [target_cpu, entry_point_addr, context_id, 0, 0, 0, 0],
        )
        .map_err(PsciError::from)?;
        Ok(())
    }

    pub fn cpu_on_64<C: SmcccCall64>(
        target_cpu: u64,
        entry_point_addr: u64,
        context_id: u64,
    ) -> Result<(), PsciError> {
        C::call(
            PSCI_CPU_ON_64,
            [
                target_cpu,
                entry_point_addr,
                context_id,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
        )
        .map_err(PsciError::from)?;

        Ok(())
    }

    pub fn affinity_info_32<C: SmcccCall32>(
        target_affinity: u32,
        lowest_affinity_level: u32,
    ) -> Result<AffinityInfo, PsciError> {
        let res = C::call(
            PSCI_AFFINITY_INFO_32,
            [target_affinity, lowest_affinity_level, 0, 0, 0, 0, 0],
        )
        .map_err(PsciError::from)?;

        Ok(AffinityInfo::from(res[0]))
    }

    pub fn affinity_info_64<C: SmcccCall64>(
        target_affinity: u64,
        lowest_affinity_level: u32,
    ) -> Result<AffinityInfo, PsciError> {
        let res = C::call(
            PSCI_AFFINITY_INFO_64,
            [
                target_affinity,
                lowest_affinity_level as u64,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
        )
        .map_err(PsciError::from)?;

        Ok(AffinityInfo::from(res[0]))
    }

    pub fn migrate_32<C: SmcccCall32>(target_cpu: u32) -> Result<AffinityInfo, PsciError> {
        let res =
            C::call(PSCI_MIGRATE_32, [target_cpu, 0, 0, 0, 0, 0, 0]).map_err(PsciError::from)?;

        Ok(AffinityInfo::from(res[0]))
    }

    pub fn migrate_64<C: SmcccCall64>(target_cpu: u64) -> Result<AffinityInfo, PsciError> {
        let res = C::call(
            PSCI_MIGRATE_64,
            [target_cpu, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        )
        .map_err(PsciError::from)?;

        Ok(AffinityInfo::from(res[0]))
    }

    pub fn migrate_info_type_32<C: SmcccCall32>() -> Result<MigrateInfoType, PsciError> {
        let res = C::call(PSCI_MIGRATE_INFO_TYPE, [0; 7]).map_err(PsciError::from)?;

        Ok(MigrateInfoType::from(res[0]))
    }

    pub fn migrate_info_up_cpu_32<C: SmcccCall32>() -> Result<u32, PsciError> {
        let res = C::call(PSCI_MIGRATE_INFO_UP_CPU_32, [0; 7]).map_err(PsciError::from)?;

        Ok(res[0])
    }

    pub fn migrate_info_up_cpu_64<C: SmcccCall64>() -> Result<u64, PsciError> {
        let res = C::call(PSCI_MIGRATE_INFO_UP_CPU_64, [0; 17]).map_err(PsciError::from)?;

        Ok(res[0])
    }

    pub fn system_off<C: SmcccCall32>() -> Result<(), PsciError> {
        C::call(PSCI_SYSTEM_OFF, [0; 7]).map_err(PsciError::from)?;

        Ok(())
    }

    pub fn system_reset<C: SmcccCall32>() -> Result<(), PsciError> {
        C::call(PSCI_SYSTEM_RESET, [0; 7]).map_err(PsciError::from)?;

        Ok(())
    }

    pub fn system_reset2_32<C: SmcccCall32>(reset_type: u32, cookie: u32) -> Result<(), PsciError> {
        C::call(PSCI_SYSTEM_RESET2_32, [reset_type, cookie, 0, 0, 0, 0, 0])
            .map_err(PsciError::from)?;

        Ok(())
    }

    pub fn system_reset2_64<C: SmcccCall64>(reset_type: u32, cookie: u64) -> Result<(), PsciError> {
        C::call(
            PSCI_SYSTEM_RESET2_64,
            [
                reset_type as u64,
                cookie,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
        )
        .map_err(PsciError::from)?;

        Ok(())
    }

    pub fn mem_protect<C: SmcccCall32>(enable: bool) -> Result<u32, PsciError> {
        let res = C::call(PSCI_MEM_PROTECT, [enable as u32, 0, 0, 0, 0, 0, 0])
            .map_err(PsciError::from)?;

        Ok(res[0])
    }

    pub fn mem_protect_check_range_32<C: SmcccCall32>(
        base: u32,
        length: u32,
    ) -> Result<(), PsciError> {
        C::call(
            PSCI_MEM_PROTECT_CHECK_RANGE_32,
            [base, length, 0, 0, 0, 0, 0],
        )
        .map_err(PsciError::from)?;

        Ok(())
    }

    pub fn mem_protect_check_range_64<C: SmcccCall64>(
        base: u64,
        length: u64,
    ) -> Result<(), PsciError> {
        C::call(
            PSCI_MEM_PROTECT_CHECK_RANGE_64,
            [base, length, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        )
        .map_err(PsciError::from)?;

        Ok(())
    }

    pub fn psci_features<C: SmcccCall32>(psci_func_id: u32) -> Result<u32, PsciError> {
        let res =
            C::call(PSCI_FEATURES, [psci_func_id, 0, 0, 0, 0, 0, 0]).map_err(PsciError::from)?;

        Ok(res[0])
    }

    pub fn cpu_freeze<C: SmcccCall32>() -> Result<(), PsciError> {
        C::call(PSCI_CPU_FREEZE, [0; 7]).map_err(PsciError::from)?;

        Ok(())
    }

    pub fn cpu_default_suspend_32<C: SmcccCall32>(
        entry_point_addr: u32,
        context_id: u32,
    ) -> Result<(), PsciError> {
        C::call(
            PSCI_CPU_DEFAULT_SUSPEND_32,
            [entry_point_addr, context_id, 0, 0, 0, 0, 0],
        )
        .map_err(PsciError::from)?;

        Ok(())
    }

    pub fn cpu_default_suspend_64<C: SmcccCall64>(
        entry_point_addr: u64,
        context_id: u64,
    ) -> Result<(), PsciError> {
        C::call(
            PSCI_CPU_DEFAULT_SUSPEND_64,
            [
                entry_point_addr,
                context_id,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
        )
        .map_err(PsciError::from)?;

        Ok(())
    }

    pub fn node_hw_state_32<C: SmcccCall32>(
        target_cpu: u32,
        power_level: u32,
    ) -> Result<NodeHwState, PsciError> {
        let res = C::call(
            PSCI_NODE_HW_STATE_32,
            [target_cpu, power_level, 0, 0, 0, 0, 0],
        )
        .map_err(PsciError::from)?;

        Ok(NodeHwState::from(res[0]))
    }

    pub fn node_hw_state_64<C: SmcccCall64>(
        target_cpu: u64,
        power_level: u32,
    ) -> Result<NodeHwState, PsciError> {
        let res = C::call(
            PSCI_NODE_HW_STATE_64,
            [
                target_cpu,
                power_level as u64,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
        )
        .map_err(PsciError::from)?;

        Ok(NodeHwState::from(res[0]))
    }

    pub fn system_suspend_32<C: SmcccCall32>(
        entry_point_addr: u32,
        context_id: u32,
    ) -> Result<(), PsciError> {
        C::call(
            PSCI_SYSTEM_SUSPEND_32,
            [entry_point_addr, context_id, 0, 0, 0, 0, 0],
        )
        .map_err(PsciError::from)?;

        Ok(())
    }

    pub fn system_suspend_64<C: SmcccCall64>(
        entry_point_addr: u64,
        context_id: u64,
    ) -> Result<(), PsciError> {
        C::call(
            PSCI_SYSTEM_SUSPEND_64,
            [
                entry_point_addr,
                context_id,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
        )
        .map_err(PsciError::from)?;

        Ok(())
    }

    pub fn psci_set_suspend_mode<C: SmcccCall32>(mode: SuspendMode) -> Result<(), PsciError> {
        C::call(PSCI_SET_SUSPEND_MODE, [mode.into(), 0, 0, 0, 0, 0, 0]).map_err(PsciError::from)?;

        Ok(())
    }

    pub fn psci_stat_residency_32<C: SmcccCall32>(
        target_cpu: u32,
        power_state: u32,
    ) -> Result<Duration, PsciError> {
        let res = C::call(
            PSCI_STAT_RESIDENCY_32,
            [target_cpu, power_state, 0, 0, 0, 0, 0],
        )
        .map_err(PsciError::from)?;

        Ok(Duration::from_micros(res[0] as u64))
    }

    pub fn psci_stat_residency_64<C: SmcccCall64>(
        target_cpu: u64,
        power_state: u32,
    ) -> Result<Duration, PsciError> {
        let res = C::call(
            PSCI_STAT_RESIDENCY_64,
            [
                target_cpu,
                power_state as u64,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
        )
        .map_err(PsciError::from)?;

        Ok(Duration::from_micros(res[0]))
    }

    pub fn psci_stat_count_32<C: SmcccCall32>(
        target_cpu: u32,
        power_state: u32,
    ) -> Result<u32, PsciError> {
        let res = C::call(PSCI_STAT_COUNT_32, [target_cpu, power_state, 0, 0, 0, 0, 0])
            .map_err(PsciError::from)?;

        Ok(res[0])
    }

    pub fn psci_stat_count_64<C: SmcccCall64>(
        target_cpu: u64,
        power_state: u32,
    ) -> Result<u64, PsciError> {
        let res: [u64; 18] = C::call(
            PSCI_STAT_COUNT_64,
            [
                target_cpu,
                power_state as u64,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
        )
        .map_err(PsciError::from)?;

        Ok(res[0])
    }
}

#[derive(Debug)]
pub enum PsciError {
    NotSupported,
    InvalidParameters,
    Denied,
    AlreadyOn,
    OnPending,
    InternalFailure,
    NotPresent,
    Disabled,
    InvalidAddress,
    Other(i64),
}

impl From<i32> for PsciError {
    fn from(value: i32) -> Self {
        match value {
            -1 => PsciError::NotSupported,
            -2 => PsciError::InvalidParameters,
            -3 => PsciError::Denied,
            -4 => PsciError::AlreadyOn,
            -5 => PsciError::OnPending,
            -6 => PsciError::InternalFailure,
            -7 => PsciError::NotPresent,
            -8 => PsciError::Disabled,
            -9 => PsciError::InvalidAddress,
            _ => PsciError::Other(value as i64),
        }
    }
}

impl From<i64> for PsciError {
    fn from(value: i64) -> Self {
        match value {
            -1 => PsciError::NotSupported,
            -2 => PsciError::InvalidParameters,
            -3 => PsciError::Denied,
            -4 => PsciError::AlreadyOn,
            -5 => PsciError::OnPending,
            -6 => PsciError::InternalFailure,
            -7 => PsciError::NotPresent,
            -8 => PsciError::Disabled,
            -9 => PsciError::InvalidAddress,
            _ => PsciError::Other(value),
        }
    }
}

pub struct PsciVersion {
    pub major: usize,
    pub minor: usize,
}

pub enum AffinityInfo {
    On,
    Off,
    OnPending,
    Other(u64),
}

impl From<u32> for AffinityInfo {
    fn from(value: u32) -> Self {
        match value {
            0 => AffinityInfo::On,
            1 => AffinityInfo::Off,
            2 => AffinityInfo::OnPending,
            other => AffinityInfo::Other(other as u64),
        }
    }
}

impl From<u64> for AffinityInfo {
    fn from(value: u64) -> Self {
        match value {
            0 => AffinityInfo::On,
            1 => AffinityInfo::Off,
            2 => AffinityInfo::OnPending,
            other => AffinityInfo::Other(other),
        }
    }
}

pub enum MigrateInfoType {
    UniprocessorMigrateCapable,
    UniprocessorNotMigrateCapable,
    MigrateNotRequired,
    Other(u32),
}

impl From<u32> for MigrateInfoType {
    fn from(value: u32) -> Self {
        match value {
            0 => MigrateInfoType::UniprocessorMigrateCapable,
            1 => MigrateInfoType::UniprocessorNotMigrateCapable,
            2 => MigrateInfoType::MigrateNotRequired,
            other => MigrateInfoType::Other(other),
        }
    }
}

pub enum NodeHwState {
    HwOn,
    HwOff,
    HwStandby,
    Other(u64),
}

impl From<u32> for NodeHwState {
    fn from(value: u32) -> Self {
        match value {
            0 => NodeHwState::HwOn,
            1 => NodeHwState::HwOff,
            2 => NodeHwState::HwStandby,
            other => NodeHwState::Other(other as u64),
        }
    }
}

impl From<u64> for NodeHwState {
    fn from(value: u64) -> Self {
        match value {
            0 => NodeHwState::HwOn,
            1 => NodeHwState::HwOff,
            2 => NodeHwState::HwStandby,
            other => NodeHwState::Other(other),
        }
    }
}

pub enum SuspendMode {
    Platform,
    OS,
}

impl From<SuspendMode> for u32 {
    fn from(value: SuspendMode) -> Self {
        match value {
            SuspendMode::Platform => 0,
            SuspendMode::OS => 1,
        }
    }
}

pub const PSCI_VERSION: u32 = 0x84000000;
pub const PSCI_CPU_SUSPEND_32: u32 = 0x84000001;
pub const PSCI_CPU_SUSPEND_64: u32 = 0xC4000001;
pub const PSCI_CPU_OFF: u32 = 0x84000002;
pub const PSCI_CPU_ON_32: u32 = 0x84000003;
pub const PSCI_CPU_ON_64: u32 = 0xC4000003;
pub const PSCI_AFFINITY_INFO_32: u32 = 0x84000004;
pub const PSCI_AFFINITY_INFO_64: u32 = 0xC4000004;
pub const PSCI_MIGRATE_32: u32 = 0x84000005;
pub const PSCI_MIGRATE_64: u32 = 0xC4000005;
pub const PSCI_MIGRATE_INFO_TYPE: u32 = 0x84000006;
pub const PSCI_MIGRATE_INFO_UP_CPU_32: u32 = 0x84000007;
pub const PSCI_MIGRATE_INFO_UP_CPU_64: u32 = 0xC4000007;
pub const PSCI_SYSTEM_OFF: u32 = 0x84000008;
pub const PSCI_SYSTEM_RESET: u32 = 0x84000009;
pub const PSCI_SYSTEM_RESET2_32: u32 = 0x84000012;
pub const PSCI_SYSTEM_RESET2_64: u32 = 0xC4000012;
pub const PSCI_MEM_PROTECT: u32 = 0x84000013;
pub const PSCI_MEM_PROTECT_CHECK_RANGE_32: u32 = 0x84000014;
pub const PSCI_MEM_PROTECT_CHECK_RANGE_64: u32 = 0xC4000014;
pub const PSCI_FEATURES: u32 = 0x8400000A;
pub const PSCI_CPU_FREEZE: u32 = 0x8400000B;
pub const PSCI_CPU_DEFAULT_SUSPEND_32: u32 = 0x8400000C;
pub const PSCI_CPU_DEFAULT_SUSPEND_64: u32 = 0xC400000C;
pub const PSCI_NODE_HW_STATE_32: u32 = 0x8400000D;
pub const PSCI_NODE_HW_STATE_64: u32 = 0xC400000D;
pub const PSCI_SYSTEM_SUSPEND_32: u32 = 0x8400000E;
pub const PSCI_SYSTEM_SUSPEND_64: u32 = 0xC400000E;
pub const PSCI_SET_SUSPEND_MODE: u32 = 0x8400000F;
pub const PSCI_STAT_RESIDENCY_32: u32 = 0x84000010;
pub const PSCI_STAT_RESIDENCY_64: u32 = 0xC4000010;
pub const PSCI_STAT_COUNT_32: u32 = 0x84000011;
pub const PSCI_STAT_COUNT_64: u32 = 0xC4000011;
