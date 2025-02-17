use crate::account;
use crate::chain::API_CLIENT_POOL;
use crate::model::dao::WithGov;

use super::base_hander::BaseHander;
use super::{super::client::Client, wetee_gov::run_sudo_or_gov};
use sp_core::crypto::Ss58Codec;
use sp_core::sr25519;
use sp_runtime::AccountId32;
use wetee_dao::GuildInfo;
use wetee_runtime::{AccountId, BlockNumber, Runtime, RuntimeCall, Signature, WeteeGuildCall};

use substrate_api_client::{ExtrinsicSigner, GetStorage, SubmitAndWatchUntilSuccess};

/// 账户
pub struct WeteeGuild {
    pub base: BaseHander,
}

impl WeteeGuild {
    pub fn new(c: Client) -> Self {
        Self {
            base: BaseHander::new(c, false),
        }
    }

    pub fn guild_list(
        &mut self,
        dao_id: u64,
    ) -> anyhow::Result<Vec<GuildInfo<AccountId, BlockNumber>>, anyhow::Error> {
        let pool = API_CLIENT_POOL.lock().unwrap();
        let api = pool.get(self.base.client.index).unwrap();

        // 构建请求
        let result: Vec<GuildInfo<AccountId, BlockNumber>> = api
            .get_storage_map("WeteeDAO", "Guilds", dao_id, None)
            .unwrap()
            .unwrap_or_else(|| vec![]);

        Ok(result)
    }

    pub fn guild_info(
        &mut self,
        dao_id: u64,
        index: u32,
    ) -> anyhow::Result<GuildInfo<AccountId, BlockNumber>, anyhow::Error> {
        let pool = API_CLIENT_POOL.lock().unwrap();
        let api = pool.get(self.base.client.index).unwrap();

        // 构建请求
        let result: Vec<GuildInfo<AccountId, BlockNumber>> = api
            .get_storage_map("WeteeDAO", "Guilds", dao_id, None)
            .unwrap()
            .unwrap_or_else(|| vec![]);

        Ok(result.get(index as usize).unwrap().clone())
    }

    pub fn create_guild(
        &mut self,
        from: String,
        dao_id: u64,
        name: String,
        desc: String,
        meta_data: String,
        ext: Option<WithGov>,
    ) -> anyhow::Result<(), anyhow::Error> {
        // 构建请求
        let who: AccountId32 = sr25519::Public::from_string(&from).unwrap().into();
        let call = RuntimeCall::WeteeGuild(WeteeGuildCall::create_guild {
            name: name.into(),
            desc: desc.into(),
            meta_data: meta_data.into(),
            dao_id,
            creator: who,
        });

        if ext.is_some() {
            return run_sudo_or_gov(self.base.client.index, from, dao_id, call, ext.unwrap());
        }

        let mut pool = API_CLIENT_POOL.lock().unwrap();
        let api = pool.get_mut(self.base.client.index).unwrap();

        let from_pair = account::get_from_address(from.clone())?;
        api.set_signer(ExtrinsicSigner::<_, Signature, Runtime>::new(from_pair));

        let signer_nonce = api.get_nonce().unwrap();
        let xt = api.compose_extrinsic_offline(call, signer_nonce);

        // 发送请求
        let result = api.submit_and_watch_extrinsic_until_success(xt, false);

        match result {
            Ok(report) => {
                println!(
                    "[+] Extrinsic got included in block {:?}",
                    report.block_hash
                );
                return Ok(());
            }
            Err(e) => {
                println!("[+] Couldn't execute the extrinsic due to {:?}\n", e);
                let string_error = format!("{:?}", e);
                return Err(anyhow::anyhow!(string_error));
            }
        };
    }

    // 成员列表
    pub fn member_list(
        &mut self,
        dao_id: u64,
        guild_id: u64,
    ) -> anyhow::Result<Vec<AccountId>, anyhow::Error> {
        let pool = API_CLIENT_POOL.lock().unwrap();
        let api = pool.get(self.base.client.index).unwrap();

        // 构建请求
        let result: Vec<AccountId> = api
            .get_storage_double_map("WeteeDAO", "GuildMembers", dao_id, guild_id, None)
            .unwrap()
            .unwrap_or_else(|| vec![]);

        Ok(result)
    }

    pub fn guild_join_request(
        &mut self,
        from: String,
        dao_id: u64,
        guild_id: u64,
        ext: Option<WithGov>,
    ) -> anyhow::Result<(), anyhow::Error> {
        // 构建请求
        let who: AccountId32 = sr25519::Public::from_string(&from).unwrap().into();
        let call = RuntimeCall::WeteeGuild(WeteeGuildCall::guild_join_request {
            dao_id,
            guild_id,
            who,
        });
        if ext.is_some() {
            return run_sudo_or_gov(self.base.client.index, from, dao_id, call, ext.unwrap());
        }

        let mut pool = API_CLIENT_POOL.lock().unwrap();
        let api = pool.get_mut(self.base.client.index).unwrap();

        let from_pair = account::get_from_address(from.clone())?;
        api.set_signer(ExtrinsicSigner::<_, Signature, Runtime>::new(from_pair));

        let signer_nonce = api.get_nonce().unwrap();
        let xt = api.compose_extrinsic_offline(call, signer_nonce);

        // 发送请求
        let result = api.submit_and_watch_extrinsic_until_success(xt, false);

        match result {
            Ok(report) => {
                println!(
                    "[+] Extrinsic got included in block {:?}",
                    report.block_hash
                );
                return Ok(());
            }
            Err(e) => {
                println!("[+] Couldn't execute the extrinsic due to {:?}\n", e);
                let string_error = format!("{:?}", e);
                return Err(anyhow::anyhow!(string_error));
            }
        };
    }
}
