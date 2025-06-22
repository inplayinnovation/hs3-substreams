mod idl;
mod pb;

use anchor_lang::AnchorDeserialize;
use anchor_lang::Discriminator;
use base64::prelude::*;
use pb::substreams::v1::program::Data;
use pb::substreams::v1::program::WagerInitializedEvent;
use pb::substreams::v1::program::WagerInitializedV2Event;
use pb::substreams::v1::program::WagerResolvedEvent;
use pb::substreams::v1::program::WagerResolvedV2Event;
use pb::substreams::v1::program::FinalizeWagerV2Instruction;
use pb::substreams::v1::program::InitializeWagerInstruction;
use pb::substreams::v1::program::InitializeWagerV2Instruction;
use pb::substreams::v1::program::ResolveWagerInstruction;
use pb::substreams::v1::program::ResolveWagerV2Instruction;









use sologger_log_context::programs_selector::ProgramsSelector;
use sologger_log_context::sologger_log_context::LogContext;
use substreams_solana::pb::sf::solana::r#type::v1::Block;

const PROGRAM_ID: &str = "6xw9f54fZrCFyhRtNBfA9tzuvwzUfe4DUw33JD1uyyfd";

#[substreams::handlers::map]
fn map_program_data(blk: Block) -> Data {
    let mut wager_initialized_event_list: Vec<WagerInitializedEvent> = Vec::new();
    let mut wager_initialized_v2_event_list: Vec<WagerInitializedV2Event> = Vec::new();
    let mut wager_resolved_event_list: Vec<WagerResolvedEvent> = Vec::new();
    let mut wager_resolved_v2_event_list: Vec<WagerResolvedV2Event> = Vec::new();
    let mut finalize_wager_v2_instruction_list: Vec<FinalizeWagerV2Instruction> = Vec::new();
    let mut initialize_wager_instruction_list: Vec<InitializeWagerInstruction> = Vec::new();
    let mut initialize_wager_v2_instruction_list: Vec<InitializeWagerV2Instruction> = Vec::new();
    let mut resolve_wager_instruction_list: Vec<ResolveWagerInstruction> = Vec::new();
    let mut resolve_wager_v2_instruction_list: Vec<ResolveWagerV2Instruction> = Vec::new();

    blk.transactions().for_each(|transaction| {

        // ------------- EVENTS -------------
        let meta_wrapped = &transaction.meta;
        let meta = meta_wrapped.as_ref().unwrap();
        let programs_selector: ProgramsSelector = ProgramsSelector::new(&["*".to_string()]);
        let log_contexts = LogContext::parse_logs_basic(&meta.log_messages, &programs_selector);

        log_contexts
            .iter()
            .filter(|context| context.program_id == PROGRAM_ID)
            .for_each(|context| {
                context.data_logs.iter().for_each(|data| {
                    if let Ok(decoded) = BASE64_STANDARD.decode(data) {
                        let slice_u8: &mut &[u8] = &mut &decoded[..];
                        let slice_discriminator: [u8; 8] =
                            slice_u8[0..8].try_into().expect("error");
                        let static_discriminator_slice: &'static [u8] = Box::leak(Box::new(slice_discriminator));

                        match static_discriminator_slice {
                            idl::idl::program::events::WagerInitialized::DISCRIMINATOR => {
                                if let Ok(event) =
                                    idl::idl::program::events::WagerInitialized::deserialize(
                                        &mut &slice_u8[8..],
                                    )
                                {
                                    wager_initialized_event_list.push(WagerInitializedEvent {
                                        trx_hash: transaction.id(),
                                        wager: event.wager.to_string(),
                                        user: event.user.to_string(),
                                        operator: event.operator.to_string(),
                                        mint: event.mint.to_string(),
                                        amount: event.amount,
                                        timestamp: event.timestamp,
                                    });
                                }
                            }
                            idl::idl::program::events::WagerInitializedV2::DISCRIMINATOR => {
                                if let Ok(event) =
                                    idl::idl::program::events::WagerInitializedV2::deserialize(
                                        &mut &slice_u8[8..],
                                    )
                                {
                                    wager_initialized_v2_event_list.push(WagerInitializedV2Event {
                                        trx_hash: transaction.id(),
                                        wager: event.wager.to_string(),
                                        user: event.user.to_string(),
                                        operator: event.operator.to_string(),
                                        wager_mint: event.wager_mint.to_string(),
                                        payout_mint: event.payout_mint.to_string(),
                                        ipfs_cid: event.ipfs_cid,
                                        wager_amount: event.wager_amount,
                                        timestamp: event.timestamp,
                                    });
                                }
                            }
                            idl::idl::program::events::WagerResolved::DISCRIMINATOR => {
                                if let Ok(event) =
                                    idl::idl::program::events::WagerResolved::deserialize(
                                        &mut &slice_u8[8..],
                                    )
                                {
                                    wager_resolved_event_list.push(WagerResolvedEvent {
                                        trx_hash: transaction.id(),
                                        wager: event.wager.to_string(),
                                        user: event.user.to_string(),
                                        operator: event.operator.to_string(),
                                        mint: event.mint.to_string(),
                                        payout_amount: event.payout_amount,
                                        timestamp: event.timestamp,
                                    });
                                }
                            }
                            idl::idl::program::events::WagerResolvedV2::DISCRIMINATOR => {
                                if let Ok(event) =
                                    idl::idl::program::events::WagerResolvedV2::deserialize(
                                        &mut &slice_u8[8..],
                                    )
                                {
                                    wager_resolved_v2_event_list.push(WagerResolvedV2Event {
                                        trx_hash: transaction.id(),
                                        wager: event.wager.to_string(),
                                        user: event.user.to_string(),
                                        operator: event.operator.to_string(),
                                        wager_mint: event.wager_mint.to_string(),
                                        payout_mint: event.payout_mint.to_string(),
                                        wager_amount: event.wager_amount,
                                        payout_amount: event.payout_amount,
                                        timestamp: event.timestamp,
                                    });
                                }
                            }
                            _ => {}
                        }
                    }
                });
            });// ------------- INSTRUCTIONS -------------
        transaction
        .walk_instructions()
        .into_iter()
        .filter(|inst| inst.program_id().to_string() == PROGRAM_ID)
        .for_each(|inst| {
            let slice_u8: &[u8] = &inst.data()[..];

            /*
                CPI events are contained inside the instruction data
            */
            if slice_u8.len() >= 16 {
                if &slice_u8[8..16] == idl::idl::program::events::WagerInitialized::DISCRIMINATOR {
                    if let Ok(event) =
                        idl::idl::program::events::WagerInitialized::deserialize(
                            &mut &slice_u8[16..],
                        )
                    {
                        wager_initialized_event_list.push(WagerInitializedEvent {
                            trx_hash: transaction.id(),
                            wager: event.wager.to_string(),
                            user: event.user.to_string(),
                            operator: event.operator.to_string(),
                            mint: event.mint.to_string(),
                            amount: event.amount,
                            timestamp: event.timestamp,
                        });
                    }
                }
                if &slice_u8[8..16] == idl::idl::program::events::WagerInitializedV2::DISCRIMINATOR {
                    if let Ok(event) =
                        idl::idl::program::events::WagerInitializedV2::deserialize(
                            &mut &slice_u8[16..],
                        )
                    {
                        wager_initialized_v2_event_list.push(WagerInitializedV2Event {
                            trx_hash: transaction.id(),
                            wager: event.wager.to_string(),
                            user: event.user.to_string(),
                            operator: event.operator.to_string(),
                            wager_mint: event.wager_mint.to_string(),
                            payout_mint: event.payout_mint.to_string(),
                            ipfs_cid: event.ipfs_cid,
                            wager_amount: event.wager_amount,
                            timestamp: event.timestamp,
                        });
                    }
                }
                if &slice_u8[8..16] == idl::idl::program::events::WagerResolved::DISCRIMINATOR {
                    if let Ok(event) =
                        idl::idl::program::events::WagerResolved::deserialize(
                            &mut &slice_u8[16..],
                        )
                    {
                        wager_resolved_event_list.push(WagerResolvedEvent {
                            trx_hash: transaction.id(),
                            wager: event.wager.to_string(),
                            user: event.user.to_string(),
                            operator: event.operator.to_string(),
                            mint: event.mint.to_string(),
                            payout_amount: event.payout_amount,
                            timestamp: event.timestamp,
                        });
                    }
                }
                if &slice_u8[8..16] == idl::idl::program::events::WagerResolvedV2::DISCRIMINATOR {
                    if let Ok(event) =
                        idl::idl::program::events::WagerResolvedV2::deserialize(
                            &mut &slice_u8[16..],
                        )
                    {
                        wager_resolved_v2_event_list.push(WagerResolvedV2Event {
                            trx_hash: transaction.id(),
                            wager: event.wager.to_string(),
                            user: event.user.to_string(),
                            operator: event.operator.to_string(),
                            wager_mint: event.wager_mint.to_string(),
                            payout_mint: event.payout_mint.to_string(),
                            wager_amount: event.wager_amount,
                            payout_amount: event.payout_amount,
                            timestamp: event.timestamp,
                        });
                    }
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::FinalizeWagerV2::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::FinalizeWagerV2::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    finalize_wager_v2_instruction_list.push(FinalizeWagerV2Instruction {
                        trx_hash: transaction.id(),
                        acct_wager: accts[0].to_string(),
                        acct_operator: accts[1].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::InitializeWager::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::InitializeWager::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    initialize_wager_instruction_list.push(InitializeWagerInstruction {
                        trx_hash: transaction.id(),
                        amount: instruction.amount,
                        acct_wager: accts[0].to_string(),
                        acct_user: accts[1].to_string(),
                        acct_operator: accts[2].to_string(),
                        acct_mint: accts[3].to_string(),
                        acct_user_token_account: accts[4].to_string(),
                        acct_operator_token_account: accts[5].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::InitializeWagerV2::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::InitializeWagerV2::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    initialize_wager_v2_instruction_list.push(InitializeWagerV2Instruction {
                        trx_hash: transaction.id(),
                        wager_amount: instruction.wager_amount,
                        ipfs_cid: instruction.ipfs_cid,
                        acct_wager: accts[0].to_string(),
                        acct_user: accts[1].to_string(),
                        acct_operator: accts[2].to_string(),
                        acct_wager_mint: accts[3].to_string(),
                        acct_payout_mint: accts[4].to_string(),
                        acct_user_token_account: accts[5].to_string(),
                        acct_operator_token_account: accts[6].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::ResolveWager::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::ResolveWager::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    resolve_wager_instruction_list.push(ResolveWagerInstruction {
                        trx_hash: transaction.id(),
                        payout_amount: instruction.payout_amount,
                        acct_wager: accts[0].to_string(),
                        acct_operator: accts[1].to_string(),
                        acct_user_token_account: accts[2].to_string(),
                        acct_operator_token_account: accts[3].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::ResolveWagerV2::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::ResolveWagerV2::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    resolve_wager_v2_instruction_list.push(ResolveWagerV2Instruction {
                        trx_hash: transaction.id(),
                        payout_amount: instruction.payout_amount,
                        acct_wager: accts[0].to_string(),
                        acct_operator: accts[1].to_string(),
                        acct_user_token_account: accts[2].to_string(),
                        acct_operator_token_account: accts[3].to_string(),
                    });
                }
            }
        });
    });


    Data {
        wager_initialized_event_list,
        wager_initialized_v2_event_list,
        wager_resolved_event_list,
        wager_resolved_v2_event_list,
        finalize_wager_v2_instruction_list,
        initialize_wager_instruction_list,
        initialize_wager_v2_instruction_list,
        resolve_wager_instruction_list,
        resolve_wager_v2_instruction_list,
    }
}



















