pub use limit_order_intent_book::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod limit_order_intent_book {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("cancelIntent"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("cancelIntent"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("intentId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("intentBidData"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("intentBidData"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("intentId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("bid"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("intentData"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("intentData"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("intent"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("signature"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("intentStates"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("intentStates"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("status"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "enum BaseIntentBook.IntentStatus",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("intentBidId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("matchAndSettle"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("matchAndSettle"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("intentBid"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IntentBookLib.IntentBid",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("matchIntent"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("matchIntent"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("intentBid"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IntentBookLib.IntentBid",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("placeIntent"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("placeIntent"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("intent"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IntentBookLib.Intent",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("intentId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("settleIntent"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("settleIntent"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("intentId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("verify"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("verify"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("limitOrderBid"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct LimitOrderIntentLib.LimitOrderBid",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("verifySignature"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("verifySignature"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("signature"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("limitOrder"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct LimitOrderIntentLib.LimitOrder",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("IntentBidReceived"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("IntentBidReceived"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("intentId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("intentBidId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("intentBid"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ],
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("IntentCancelled"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("IntentCancelled"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("intentId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("IntentCreated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("IntentCreated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("intentId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("intent"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ],
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("IntentMatch"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("IntentMatch"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("intentId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("intentBidId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("IntentPartiallySettled"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "IntentPartiallySettled",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("intentId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("intentBidId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("IntentSettled"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("IntentSettled"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("intentId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("intentBidId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("LimitOrderFulfilled"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "LimitOrderFulfilled",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("intentId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("LimitOrderPartialFill"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "LimitOrderPartialFill",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("intentId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("volumeFilled"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("NotImplemented"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("NotImplemented"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static LIMITORDERINTENTBOOK_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa M\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\x9EW`\x005`\xE0\x1C\x80c{\xF8\xBB\x88\x11a\0fW\x80c{\xF8\xBB\x88\x14a\x01KW\x80c\x87\xF6\x17\xB6\x14a\x01^W\x80c\xD5_\x96\r\x14a\x01qW\x80c\xE2V#\xE0\x14a\x01\x84W\x80c\xEEW\x01\xE7\x14a\x01\xA5W`\0\x80\xFD[\x80c\x03\x89\\\x91\x14a\0\xA3W\x80c\t\xC7\xB2\xF6\x14a\0\xB6W\x80cJ\xF26N\x14a\0\xC9W\x80cY\xA8D\xB4\x14a\0\xEFW\x80c_\xF8\xA6k\x14a\x01\x10W[`\0\x80\xFD[a\0\xB4a\0\xB16`\x04a\x15\xE5V[PV[\0[a\0\xB4a\0\xC46`\x04a\x167V[a\x01\xB8V[a\0\xDCa\0\xD76`\x04a\x167V[a\x03\xFFV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\x02a\0\xFD6`\x04a\x16lV[a\x058V[`@Qa\0\xE6\x92\x91\x90a\x16\xD5V[a\x01=a\x01\x1E6`\x04a\x16lV[`\0` \x81\x90R\x90\x81R`@\x90 \x80T`\x01\x90\x91\x01T`\xFF\x90\x91\x16\x90\x82V[`@Qa\0\xE6\x92\x91\x90a\x17\x19V[a\0\xB4a\x01Y6`\x04a\x16lV[a\x06dV[a\0\xB4a\x01l6`\x04a\x167V[a\x07qV[a\0\xB4a\x01\x7F6`\x04a\x16lV[a\x07\x84V[a\x01\x97a\x01\x926`\x04a\x16lV[a\x08\x92V[`@Qa\0\xE6\x92\x91\x90a\x17EV[a\0\xB4a\x01\xB36`\x04a\x17\xEBV[a\x08\xB4V[\x805`\0\x81\x81R` \x81\x90R`@\x90 `\x01\x81\x01T\x15a\x02\x1FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FIntent already has a bid\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x81T`\xFF\x16`\x03\x81\x11\x15a\x027Wa\x027a\x17\x03V[\x03a\x02|W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rt\x12[\x9D\x19[\x9D\x08\x19\x1B\xD9\\\xC8\x1B\x9B\xDD\x08\x19^\x1A\\\xDD`Z\x1B`D\x82\x01R`d\x01a\x02\x16V[`\x03\x81T`\xFF\x16`\x03\x81\x11\x15a\x02\x94Wa\x02\x94a\x17\x03V[\x03a\x02\xE1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FIntent is already cancelled\0\0\0\0\0`D\x82\x01R`d\x01a\x02\x16V[`\x02\x81T`\xFF\x16`\x03\x81\x11\x15a\x02\xF9Wa\x02\xF9a\x17\x03V[\x03a\x03BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01Rx\x12[\x9D\x19[\x9D\x08\x1A\\\xC8\x18[\x1C\x99XY\x1EH\x1C\xD9]\x1D\x1B\x19Y`:\x1B`D\x82\x01R`d\x01a\x02\x16V[`\0a\x03Ua\x03P\x85a\x18\x95V[a\n!V[`\0\x81\x81R`\x02` R`@\x90 T\x90\x91P\x15a\x03\xA9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01RqBid already exists`p\x1B`D\x82\x01R`d\x01a\x02\x16V[`\0\x81\x81R`\x02` R`@\x90 \x84\x90a\x03\xC3\x82\x82a\x1A}V[PP`\x01\x82\x01\x81\x90U`@Q\x81\x90\x84\x90\x7F\xD4\\.\x8Ek:\xE4\x1C\xA1P\xB9R\xF5\xD3b\x92\x10\x8E\xC6m\x95\x98-F\x03}\x18\\\xD9\x05\xA1\xAE\x90`\0\x90\xA3PPPPV[`\0a\x04\x12a\x04\r\x83a\x1BNV[a\n[V[\x90Pa\x04\x1D\x81a\nxV[`\0\x81\x81R` \x81\x90R`@\x81 T`\xFF\x16`\x03\x81\x11\x15a\x04@Wa\x04@a\x17\x03V[\x14a\x04\x85W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01RtIntent already exists`X\x1B`D\x82\x01R`d\x01a\x02\x16V[`@\x80Q\x80\x82\x01\x90\x91R\x80`\x01\x81R`\0` \x91\x82\x01\x81\x90R\x83\x81R\x90\x81\x90R`@\x90 \x81Q\x81T\x82\x90`\xFF\x19\x16`\x01\x83`\x03\x81\x11\x15a\x04\xC7Wa\x04\xC7a\x17\x03V[\x02\x17\x90UP` \x91\x82\x01Q`\x01\x91\x82\x01U`\0\x83\x81R\x91R`@\x90 \x82\x90a\x04\xEF\x82\x82a\x1B\xAFV[\x90PPa\x04\xFB\x81a\x0B\xBFV[\x80\x7F\\/\xF1\xA21\x9AN\xC07\x07\x9E\xD0\xFA\xCBgnj\xDE\x19\xE5\xAC\xCBR\x86F;\xF34J\xAB\xD0G\x83`@Qa\x05+\x91\x90a\x1C\xFBV[`@Q\x80\x91\x03\x90\xA2\x91\x90PV[`\x01` R`\0\x90\x81R`@\x90 \x80T\x81\x90a\x05S\x90a\x193V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x05\x7F\x90a\x193V[\x80\x15a\x05\xCCW\x80`\x1F\x10a\x05\xA1Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x05\xCCV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x05\xAFW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90\x80`\x01\x01\x80Ta\x05\xE1\x90a\x193V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x06\r\x90a\x193V[\x80\x15a\x06ZW\x80`\x1F\x10a\x06/Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x06ZV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x06=W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x82V[`\0\x81\x81R` \x81\x90R`@\x90 `\x01\x81\x01T\x80a\x06\xC4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FIntent does not have a bid\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02\x16V[`\0a\x06\xD0\x84\x83a\x0C\x87V[`\0`\x01\x80\x86\x01\x82\x90U\x84\x82R`\x02` R`@\x82 \x82\x81U\x92\x93Pa\x06\xF8\x90\x83\x01\x82a\x15 V[PP\x80\x15a\x07=W\x82T`\xFF\x19\x16`\x02\x17\x83U`@Q\x82\x90\x85\x90\x7F\xBF\x89u\x13\x9A\xEE\x07\x94\xECPWC<4\xFB\x93\x9E\x0FeZ\x87\xB0Q\xE3*:\xAE$\xA6U/N\x90`\0\x90\xA3a\x07kV[`@Q\x82\x90\x85\x90\x7F\x84oK\x93k-|\xCF_\xCB\x9F1z\xB7\x91\xF5\xEC\xE5a\x11\x1E\x890n\x99}\x88\xBB\x84*<S\x90`\0\x90\xA3[PPPPV[a\x07z\x81a\x01\xB8V[a\0\xB1\x815a\x06dV[`\0\x81\x81R` \x81\x90R`@\x90 `\x02\x81T`\xFF\x16`\x03\x81\x11\x15a\x07\xAAWa\x07\xAAa\x17\x03V[\x03a\x07\xF3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01Rx\x12[\x9D\x19[\x9D\x08\x1A\\\xC8\x18[\x1C\x99XY\x1EH\x1C\xD9]\x1D\x1B\x19Y`:\x1B`D\x82\x01R`d\x01a\x02\x16V[`\x03\x81T`\xFF\x16`\x03\x81\x11\x15a\x08\x0BWa\x08\x0Ba\x17\x03V[\x03a\x08XW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FIntent is already cancelled\0\0\0\0\0`D\x82\x01R`d\x01a\x02\x16V[\x80T`\xFF\x19\x16`\x03\x17\x81U`@Q\x82\x90\x7F\xC0\x8E\xB6M\xB1j9\xD2\x84\x89`\xAF\x04\xE3\xF1o\xB4\x04\xD9\xD46\xA9\xF0\xE9\xD7\xD0\xD4\x85G\x15\xC9\xDC\x90`\0\x90\xA2PPV[`\x02` R`\0\x90\x81R`@\x90 \x80T`\x01\x82\x01\x80T\x91\x92\x91a\x05\xE1\x90a\x193V[`@\x80Q\x7F\xC2\xF8xqv\xB8\xACk\xF7![J\xDC\xC1\xE0i\xBFJ\xB8-\x9A\xB1\xDF\x05\xA5z\x91\xD4%\x93[n` \x80\x83\x01\x91\x90\x91R\x7F\x97\xBC\x18\x0B\x88[\xB1\x106\xFACp\xBB\x14Lj\xE5U\x8B\xEE\x80\xA1\xF0\x8B\xEF\x9C\xEFq\xAE \x04\x8A\x82\x84\x01R\x7F\x06\xC0\x15\xBD\"\xB4\xC6\x96\x90\x93<\x10X\x87\x8E\xBD\xFE\xF3\x1F\x9A\xAA\xE4\x0B\xBE\x86\xD8\xA0\x9F\xE1\xB2\x97,``\x80\x84\x01\x91\x90\x91RF`\x80\x80\x85\x01\x91\x90\x91R\x84Q\x80\x85\x03\x82\x01\x81R`\xA0\x85\x01\x86R\x80Q\x90\x84\x01 \x86Q\x84\x88\x01Q\x87\x89\x01Q\x94\x89\x01Q\x93\x89\x01Q\x7Fs0\xFFg\xE0\x989\xF9\x8D\xC5\x8Cj\x16\xA1J\xFD[\x92\xEF\xAC\x0C\xD0\xFAv\x05\x92E(zX2\x93`\xC0\x89\x01R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\xE0\x89\x01R\x90\x82\x16a\x01\0\x88\x01Ra\x01 \x87\x01\x94\x90\x94Ra\x01@\x86\x01\x92\x90\x92R\x91\x16a\x01`\x80\x85\x01\x91\x90\x91R\x84Q\x80\x85\x03\x90\x91\x01\x81Ra\x01\x80\x84\x01\x85R\x80Q\x90\x83\x01 a\x19\x01`\xF0\x1Ba\x01\xA0\x85\x01Ra\x01\xA2\x84\x01\x91\x90\x91Ra\x01\xC2\x80\x84\x01\x91\x90\x91R\x83Q\x80\x84\x03\x90\x91\x01\x81Ra\x01\xE2\x90\x92\x01\x90\x92R\x80Q\x91\x01 a\n\x1C\x83\x83`\0\x01Q\x83a\x10\x9DV[PPPV[`\0\x81`\0\x01Q\x82` \x01Q`@Q` \x01a\n>\x92\x91\x90a\x1DQV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`\0\x81`\0\x01Q\x82` \x01Q`@Q` \x01a\n>\x92\x91\x90a\x1DwV[`\0\x81\x81R`\x01` R`@\x81 \x80T\x90\x91\x90\x82\x90a\n\x96\x90a\x193V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\n\xC2\x90a\x193V[\x80\x15a\x0B\x0FW\x80`\x1F\x10a\n\xE4Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0B\x0FV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\n\xF2W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x80` \x01\x90Q\x81\x01\x90a\x0B'\x91\x90a\x1D\xA6V[\x90Pa\n\x1C\x82`\x01\x01\x80Ta\x0B;\x90a\x193V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0Bg\x90a\x193V[\x80\x15a\x0B\xB4W\x80`\x1F\x10a\x0B\x89Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0B\xB4V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0B\x97W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x82a\x08\xB4V[`\0\x81\x81R`\x01` R`@\x81 \x80T\x90\x91\x90\x82\x90a\x0B\xDD\x90a\x193V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0C\t\x90a\x193V[\x80\x15a\x0CVW\x80`\x1F\x10a\x0C+Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0CVV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0C9W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x80` \x01\x90Q\x81\x01\x90a\x0Cn\x91\x90a\x1D\xA6V[\x90Pa\n\x1C\x81` \x01Q\x82`@\x01Q\x83`\0\x01Qa\x11\x87V[`\0\x82\x81R`\x01` R`@\x81 \x80T\x82\x90\x82\x90a\x0C\xA4\x90a\x193V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0C\xD0\x90a\x193V[\x80\x15a\r\x1DW\x80`\x1F\x10a\x0C\xF2Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\r\x1DV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\r\0W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x80` \x01\x90Q\x81\x01\x90a\r5\x91\x90a\x1D\xA6V[\x90P`\0`\x02`\0\x86\x81R` \x01\x90\x81R` \x01`\0 `@Q\x80`@\x01`@R\x90\x81`\0\x82\x01T\x81R` \x01`\x01\x82\x01\x80Ta\rq\x90a\x193V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\r\x9D\x90a\x193V[\x80\x15a\r\xEAW\x80`\x1F\x10a\r\xBFWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\r\xEAV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\r\xCDW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P`\0\x81` \x01Q\x80` \x01\x90Q\x81\x01\x90a\x0E\x0F\x91\x90a\x1E\x10V[\x90P`\0\x83` \x01Q`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0EUW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Ey\x91\x90a\x1EJV[`\xFF\x16\x90P`\0\x84`\x80\x01Q`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E\xC2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\xE6\x91\x90a\x1EJV[`@\x86\x01Q``\x87\x01Q`\xFF\x92\x90\x92\x16\x92P\x90`\0g\r\xE0\xB6\xB3\xA7d\0\0a\x0F\x0F\x85`\na\x1FnV[a\x0F\x19\x90\x84a\x1FzV[a\x0F#\x91\x90a\x1F\x91V[\x90P\x87`\x80\x01Q`\x01`\x01`\xA0\x1B\x03\x16c#\xB8r\xDD3\x8A`\0\x01Q\x88`\na\x0FK\x91\x90a\x1FnV[a\x0FU\x86\x89a\x1FzV[a\x0F_\x91\x90a\x1F\x91V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x86\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16`\x04\x82\x01R\x92\x90\x91\x16`$\x83\x01R`D\x82\x01R`d\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0F\xB3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xD7\x91\x90a\x1F\xB3V[Pa\x0F\xE7\x88` \x01Q\x843a\x11\x93V[\x85` \x01Q\x88`@\x01\x81\x81Qa\x0F\xFD\x91\x90a\x1F\xD5V[\x90RP`@\x88\x01Q\x15\x80\x15a\x10DW`@Q\x8D\x81R\x7FSI^\x046\xD7\x9Dn4\xA3\x03\x0E\xE9\xEB\xE4T+\xAC\xD0\xB5\x9D>\xE5\x03\x89\x9F\xAD!K\xCC;Q\x90` \x01`@Q\x80\x91\x03\x90\xA1a\x10\x8BV[\x7FN\x03`\0H<C\xFD\xD2\xFF\x16\x1C\xE1\xD0\r\xD9\x9F*\xC8e@\x9E\xEC\xCDz\xE8\n%\xA1$?Z\x8D\x88` \x01Q`@Qa\x10\x82\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA1[\x99PPPPPPPPPP[\x92\x91PPV[`\0\x80`\0a\x10\xAB\x86a\x11\x9EV[`@\x80Q`\0\x81R` \x81\x01\x80\x83R\x89\x90R`\xFF\x85\x16\x91\x81\x01\x91\x90\x91R``\x81\x01\x83\x90R`\x80\x81\x01\x82\x90R\x92\x95P\x90\x93P\x91P`\x01`\x01`\xA0\x1B\x03\x86\x16\x90`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x11\x11W=`\0\x80>=`\0\xFD[PPP` `@Q\x03Q`\x01`\x01`\xA0\x1B\x03\x16\x14a\x11\x7FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FVerification error: Signer is in`D\x82\x01Rd\x1D\x98[\x1AY`\xDA\x1B`d\x82\x01R`\x84\x01a\x02\x16V[PPPPPPV[a\n\x1C\x83\x820\x85a\x12 V[a\n\x1C\x83\x82\x84a\x12\x8BV[`\0\x80`\0\x83Q`A\x14a\x12\x04W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FThe signature length is not equa`D\x82\x01Rfl to 65`\xC8\x1B`d\x82\x01R`\x84\x01a\x02\x16V[PPP` \x81\x01Q`@\x82\x01Q``\x90\x92\x01Q`\0\x1A\x92\x90\x91\x90V[`@Q`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`$\x83\x01R\x83\x16`D\x82\x01R`d\x81\x01\x82\x90Ra\x07k\x90\x85\x90c#\xB8r\xDD`\xE0\x1B\x90`\x84\x01[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x93\x16\x92\x90\x92\x17\x90\x91Ra\x12\xBBV[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16`$\x82\x01R`D\x81\x01\x82\x90Ra\n\x1C\x90\x84\x90c\xA9\x05\x9C\xBB`\xE0\x1B\x90`d\x01a\x12TV[`\0a\x13\x10\x82`@Q\x80`@\x01`@R\x80` \x81R` \x01\x7FSafeERC20: low-level call failed\x81RP\x85`\x01`\x01`\xA0\x1B\x03\x16a\x13\x90\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x80Q`\0\x14\x80a\x131WP\x80\x80` \x01\x90Q\x81\x01\x90a\x131\x91\x90a\x1F\xB3V[a\n\x1CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01Ri\x1B\xDD\x08\x1C\xDDX\xD8\xD9YY`\xB2\x1B`d\x82\x01R`\x84\x01a\x02\x16V[``a\x13\x9F\x84\x84`\0\x85a\x13\xA7V[\x94\x93PPPPV[``\x82G\x10\x15a\x14\x08W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FAddress: insufficient balance fo`D\x82\x01Re\x1C\x88\x18\xD8[\x1B`\xD2\x1B`d\x82\x01R`\x84\x01a\x02\x16V[`\0\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@Qa\x14$\x91\x90a\x1F\xE8V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x14aW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x14fV[``\x91P[P\x91P\x91Pa\x14w\x87\x83\x83\x87a\x14\x82V[\x97\x96PPPPPPPV[``\x83\x15a\x14\xF1W\x82Q`\0\x03a\x14\xEAW`\x01`\x01`\xA0\x1B\x03\x85\x16;a\x14\xEAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x02\x16V[P\x81a\x13\x9FV[a\x13\x9F\x83\x83\x81Q\x15a\x15\x06W\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02\x16\x91\x90a \x04V[P\x80Ta\x15,\x90a\x193V[`\0\x82U\x80`\x1F\x10a\x15<WPPV[`\x1F\x01` \x90\x04\x90`\0R` `\0 \x90\x81\x01\x90a\0\xB1\x91\x90[\x80\x82\x11\x15a\x15jW`\0\x81U`\x01\x01a\x15VV[P\x90V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x15\xA7Wa\x15\xA7a\x15nV[`@R\x90V[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x15\xA7Wa\x15\xA7a\x15nV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0\xB1W`\0\x80\xFD[`\0`@\x82\x84\x03\x12\x15a\x15\xF7W`\0\x80\xFD[a\x15\xFFa\x15\x84V[\x825a\x16\n\x81a\x15\xD0V[\x81R` \x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x91\x90PV[`\0`@\x82\x84\x03\x12\x15a\x161W`\0\x80\xFD[P\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x16IW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x16`W`\0\x80\xFD[a\x13\x9F\x84\x82\x85\x01a\x16\x1FV[`\0` \x82\x84\x03\x12\x15a\x16~W`\0\x80\xFD[P5\x91\x90PV[`\0[\x83\x81\x10\x15a\x16\xA0W\x81\x81\x01Q\x83\x82\x01R` \x01a\x16\x88V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x16\xC1\x81` \x86\x01` \x86\x01a\x16\x85V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`@\x81R`\0a\x16\xE8`@\x83\x01\x85a\x16\xA9V[\x82\x81\x03` \x84\x01Ra\x16\xFA\x81\x85a\x16\xA9V[\x95\x94PPPPPV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`@\x81\x01`\x04\x84\x10a\x17;WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x92\x81R` \x01R\x90V[\x82\x81R`@` \x82\x01R`\0a\x13\x9F`@\x83\x01\x84a\x16\xA9V[`\0\x82`\x1F\x83\x01\x12a\x17oW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x17\x8AWa\x17\x8Aa\x15nV[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x17\xB2Wa\x17\xB2a\x15nV[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\x17\xCBW`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[`\0\x80\x82\x84\x03`\xC0\x81\x12\x15a\x17\xFFW`\0\x80\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x18\x16W`\0\x80\xFD[a\x18\"\x86\x82\x87\x01a\x17^V[\x93PP`\xA0`\x1F\x19\x82\x01\x12\x15a\x187W`\0\x80\xFD[Pa\x18@a\x15\xADV[` \x84\x015a\x18N\x81a\x15\xD0V[\x81R`@\x84\x015a\x18^\x81a\x15\xD0V[` \x82\x01R``\x84\x81\x015`@\x83\x01R`\x80\x85\x015\x90\x82\x01R`\xA0\x84\x015a\x18\x85\x81a\x15\xD0V[`\x80\x82\x01R\x91\x94\x91\x93P\x90\x91PPV[`\0`@\x826\x03\x12\x15a\x18\xA7W`\0\x80\xFD[a\x18\xAFa\x15\x84V[\x825\x81R` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x18\xCDW`\0\x80\xFD[a\x18\xD96\x82\x86\x01a\x17^V[` \x83\x01RP\x92\x91PPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x18\xFCW`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x19\x17W`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x19,W`\0\x80\xFD[\x92P\x92\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x19GW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x161WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[`\x1F\x82\x11\x15a\n\x1CW`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a\x19\x8EWP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x11\x7FW\x82\x81U`\x01\x01a\x19\x9AV[`\0\x19`\x03\x83\x90\x1B\x1C\x19\x16`\x01\x91\x90\x91\x1B\x17\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x15a\x19\xDAWa\x19\xDAa\x15nV[a\x19\xEE\x83a\x19\xE8\x83Ta\x193V[\x83a\x19gV[`\0`\x1F\x84\x11`\x01\x81\x14a\x1A\x1CW`\0\x85\x15a\x1A\nWP\x83\x82\x015[a\x1A\x14\x86\x82a\x19\xADV[\x84UPa\x1AvV[`\0\x83\x81R` \x90 `\x1F\x19\x86\x16\x90\x83[\x82\x81\x10\x15a\x1AMW\x86\x85\x015\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a\x1A-V[P\x86\x82\x10\x15a\x1AjW`\0\x19`\xF8\x88`\x03\x1B\x16\x1C\x19\x84\x87\x015\x16\x81U[PP`\x01\x85`\x01\x1B\x01\x83U[PPPPPV[\x815\x81U`\x01\x80\x82\x01` a\x1A\x94\x81\x86\x01\x86a\x18\xE5V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1A\xACWa\x1A\xACa\x15nV[a\x1A\xC0\x81a\x1A\xBA\x86Ta\x193V[\x86a\x19gV[`\0`\x1F\x82\x11`\x01\x81\x14a\x1A\xEEW`\0\x83\x15a\x1A\xDCWP\x83\x82\x015[a\x1A\xE6\x84\x82a\x19\xADV[\x87UPa\x1BCV[`\0\x86\x81R` \x90 `\x1F\x19\x84\x16\x90\x83[\x82\x81\x10\x15a\x1B\x1CW\x86\x85\x015\x82U\x93\x87\x01\x93\x90\x89\x01\x90\x87\x01a\x1A\xFFV[P\x84\x82\x10\x15a\x1B9W`\0\x19`\xF8\x86`\x03\x1B\x16\x1C\x19\x84\x87\x015\x16\x81U[PP\x86\x83\x88\x1B\x01\x86U[PPPPPPPPPV[`\0`@\x826\x03\x12\x15a\x1B`W`\0\x80\xFD[a\x1Bha\x15\x84V[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x1B\x80W`\0\x80\xFD[a\x1B\x8C6\x83\x87\x01a\x17^V[\x83R` \x85\x015\x91P\x80\x82\x11\x15a\x1B\xA2W`\0\x80\xFD[Pa\x18\xD96\x82\x86\x01a\x17^V[a\x1B\xB9\x82\x83a\x18\xE5V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1B\xD1Wa\x1B\xD1a\x15nV[a\x1B\xE5\x81a\x1B\xDF\x85Ta\x193V[\x85a\x19gV[`\0`\x1F\x82\x11`\x01\x81\x14a\x1C\x13W`\0\x83\x15a\x1C\x01WP\x83\x82\x015[a\x1C\x0B\x84\x82a\x19\xADV[\x86UPa\x1CmV[`\0\x85\x81R` \x90 `\x1F\x19\x84\x16\x90\x83[\x82\x81\x10\x15a\x1CDW\x86\x85\x015\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a\x1C$V[P\x84\x82\x10\x15a\x1CaW`\0\x19`\xF8\x86`\x03\x1B\x16\x1C\x19\x84\x87\x015\x16\x81U[PP`\x01\x83`\x01\x1B\x01\x85U[PPPPa\x1C~` \x83\x01\x83a\x18\xE5V[a\x07k\x81\x83`\x01\x86\x01a\x19\xC2V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x1C\xA3W`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1C\xC3W`\0\x80\xFD[\x806\x03\x82\x13\x15a\x19,W`\0\x80\xFD[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[` \x81R`\0a\x1D\x0B\x83\x84a\x1C\x8CV[`@` \x85\x01Ra\x1D ``\x85\x01\x82\x84a\x1C\xD2V[\x91PPa\x1D0` \x85\x01\x85a\x1C\x8CV[\x84\x83\x03`\x1F\x19\x01`@\x86\x01Ra\x1DG\x83\x82\x84a\x1C\xD2V[\x96\x95PPPPPPV[\x82\x81R`\0\x82Qa\x1Di\x81` \x85\x01` \x87\x01a\x16\x85V[\x91\x90\x91\x01` \x01\x93\x92PPPV[`\0\x83Qa\x1D\x89\x81\x84` \x88\x01a\x16\x85V[\x83Q\x90\x83\x01\x90a\x1D\x9D\x81\x83` \x88\x01a\x16\x85V[\x01\x94\x93PPPPV[`\0`\xA0\x82\x84\x03\x12\x15a\x1D\xB8W`\0\x80\xFD[a\x1D\xC0a\x15\xADV[\x82Qa\x1D\xCB\x81a\x15\xD0V[\x81R` \x83\x01Qa\x1D\xDB\x81a\x15\xD0V[\x80` \x83\x01RP`@\x83\x01Q`@\x82\x01R``\x83\x01Q``\x82\x01R`\x80\x83\x01Qa\x1E\x04\x81a\x15\xD0V[`\x80\x82\x01R\x93\x92PPPV[`\0`@\x82\x84\x03\x12\x15a\x1E\"W`\0\x80\xFD[a\x1E*a\x15\x84V[\x82Qa\x1E5\x81a\x15\xD0V[\x81R` \x92\x83\x01Q\x92\x81\x01\x92\x90\x92RP\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x1E\\W`\0\x80\xFD[\x81Q`\xFF\x81\x16\x81\x14a\x1EmW`\0\x80\xFD[\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\x01\x81\x81[\x80\x85\x11\x15a\x1E\xC5W\x81`\0\x19\x04\x82\x11\x15a\x1E\xABWa\x1E\xABa\x1EtV[\x80\x85\x16\x15a\x1E\xB8W\x91\x81\x02\x91[\x93\x84\x1C\x93\x90\x80\x02\x90a\x1E\x8FV[P\x92P\x92\x90PV[`\0\x82a\x1E\xDCWP`\x01a\x10\x97V[\x81a\x1E\xE9WP`\0a\x10\x97V[\x81`\x01\x81\x14a\x1E\xFFW`\x02\x81\x14a\x1F\tWa\x1F%V[`\x01\x91PPa\x10\x97V[`\xFF\x84\x11\x15a\x1F\x1AWa\x1F\x1Aa\x1EtV[PP`\x01\x82\x1Ba\x10\x97V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a\x1FHWP\x81\x81\na\x10\x97V[a\x1FR\x83\x83a\x1E\x8AV[\x80`\0\x19\x04\x82\x11\x15a\x1FfWa\x1Ffa\x1EtV[\x02\x93\x92PPPV[`\0a\x1Em\x83\x83a\x1E\xCDV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x10\x97Wa\x10\x97a\x1EtV[`\0\x82a\x1F\xAEWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\0` \x82\x84\x03\x12\x15a\x1F\xC5W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x1EmW`\0\x80\xFD[\x81\x81\x03\x81\x81\x11\x15a\x10\x97Wa\x10\x97a\x1EtV[`\0\x82Qa\x1F\xFA\x81\x84` \x87\x01a\x16\x85V[\x91\x90\x91\x01\x92\x91PPV[` \x81R`\0a\x1Em` \x83\x01\x84a\x16\xA9V\xFE\xA2dipfsX\"\x12 \xE9\xA5\xB1m\xA8\t\x18\xE4\x96\x13)*\xE7\xC2\xAEm\\\x7F\xA8\xE7Mj\xB1\xF1@\xA2C \xAB\xB6\x0F\xDDdsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static LIMITORDERINTENTBOOK_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\x9EW`\x005`\xE0\x1C\x80c{\xF8\xBB\x88\x11a\0fW\x80c{\xF8\xBB\x88\x14a\x01KW\x80c\x87\xF6\x17\xB6\x14a\x01^W\x80c\xD5_\x96\r\x14a\x01qW\x80c\xE2V#\xE0\x14a\x01\x84W\x80c\xEEW\x01\xE7\x14a\x01\xA5W`\0\x80\xFD[\x80c\x03\x89\\\x91\x14a\0\xA3W\x80c\t\xC7\xB2\xF6\x14a\0\xB6W\x80cJ\xF26N\x14a\0\xC9W\x80cY\xA8D\xB4\x14a\0\xEFW\x80c_\xF8\xA6k\x14a\x01\x10W[`\0\x80\xFD[a\0\xB4a\0\xB16`\x04a\x15\xE5V[PV[\0[a\0\xB4a\0\xC46`\x04a\x167V[a\x01\xB8V[a\0\xDCa\0\xD76`\x04a\x167V[a\x03\xFFV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\x02a\0\xFD6`\x04a\x16lV[a\x058V[`@Qa\0\xE6\x92\x91\x90a\x16\xD5V[a\x01=a\x01\x1E6`\x04a\x16lV[`\0` \x81\x90R\x90\x81R`@\x90 \x80T`\x01\x90\x91\x01T`\xFF\x90\x91\x16\x90\x82V[`@Qa\0\xE6\x92\x91\x90a\x17\x19V[a\0\xB4a\x01Y6`\x04a\x16lV[a\x06dV[a\0\xB4a\x01l6`\x04a\x167V[a\x07qV[a\0\xB4a\x01\x7F6`\x04a\x16lV[a\x07\x84V[a\x01\x97a\x01\x926`\x04a\x16lV[a\x08\x92V[`@Qa\0\xE6\x92\x91\x90a\x17EV[a\0\xB4a\x01\xB36`\x04a\x17\xEBV[a\x08\xB4V[\x805`\0\x81\x81R` \x81\x90R`@\x90 `\x01\x81\x01T\x15a\x02\x1FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FIntent already has a bid\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x81T`\xFF\x16`\x03\x81\x11\x15a\x027Wa\x027a\x17\x03V[\x03a\x02|W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rt\x12[\x9D\x19[\x9D\x08\x19\x1B\xD9\\\xC8\x1B\x9B\xDD\x08\x19^\x1A\\\xDD`Z\x1B`D\x82\x01R`d\x01a\x02\x16V[`\x03\x81T`\xFF\x16`\x03\x81\x11\x15a\x02\x94Wa\x02\x94a\x17\x03V[\x03a\x02\xE1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FIntent is already cancelled\0\0\0\0\0`D\x82\x01R`d\x01a\x02\x16V[`\x02\x81T`\xFF\x16`\x03\x81\x11\x15a\x02\xF9Wa\x02\xF9a\x17\x03V[\x03a\x03BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01Rx\x12[\x9D\x19[\x9D\x08\x1A\\\xC8\x18[\x1C\x99XY\x1EH\x1C\xD9]\x1D\x1B\x19Y`:\x1B`D\x82\x01R`d\x01a\x02\x16V[`\0a\x03Ua\x03P\x85a\x18\x95V[a\n!V[`\0\x81\x81R`\x02` R`@\x90 T\x90\x91P\x15a\x03\xA9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01RqBid already exists`p\x1B`D\x82\x01R`d\x01a\x02\x16V[`\0\x81\x81R`\x02` R`@\x90 \x84\x90a\x03\xC3\x82\x82a\x1A}V[PP`\x01\x82\x01\x81\x90U`@Q\x81\x90\x84\x90\x7F\xD4\\.\x8Ek:\xE4\x1C\xA1P\xB9R\xF5\xD3b\x92\x10\x8E\xC6m\x95\x98-F\x03}\x18\\\xD9\x05\xA1\xAE\x90`\0\x90\xA3PPPPV[`\0a\x04\x12a\x04\r\x83a\x1BNV[a\n[V[\x90Pa\x04\x1D\x81a\nxV[`\0\x81\x81R` \x81\x90R`@\x81 T`\xFF\x16`\x03\x81\x11\x15a\x04@Wa\x04@a\x17\x03V[\x14a\x04\x85W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01RtIntent already exists`X\x1B`D\x82\x01R`d\x01a\x02\x16V[`@\x80Q\x80\x82\x01\x90\x91R\x80`\x01\x81R`\0` \x91\x82\x01\x81\x90R\x83\x81R\x90\x81\x90R`@\x90 \x81Q\x81T\x82\x90`\xFF\x19\x16`\x01\x83`\x03\x81\x11\x15a\x04\xC7Wa\x04\xC7a\x17\x03V[\x02\x17\x90UP` \x91\x82\x01Q`\x01\x91\x82\x01U`\0\x83\x81R\x91R`@\x90 \x82\x90a\x04\xEF\x82\x82a\x1B\xAFV[\x90PPa\x04\xFB\x81a\x0B\xBFV[\x80\x7F\\/\xF1\xA21\x9AN\xC07\x07\x9E\xD0\xFA\xCBgnj\xDE\x19\xE5\xAC\xCBR\x86F;\xF34J\xAB\xD0G\x83`@Qa\x05+\x91\x90a\x1C\xFBV[`@Q\x80\x91\x03\x90\xA2\x91\x90PV[`\x01` R`\0\x90\x81R`@\x90 \x80T\x81\x90a\x05S\x90a\x193V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x05\x7F\x90a\x193V[\x80\x15a\x05\xCCW\x80`\x1F\x10a\x05\xA1Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x05\xCCV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x05\xAFW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90\x80`\x01\x01\x80Ta\x05\xE1\x90a\x193V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x06\r\x90a\x193V[\x80\x15a\x06ZW\x80`\x1F\x10a\x06/Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x06ZV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x06=W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x82V[`\0\x81\x81R` \x81\x90R`@\x90 `\x01\x81\x01T\x80a\x06\xC4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FIntent does not have a bid\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02\x16V[`\0a\x06\xD0\x84\x83a\x0C\x87V[`\0`\x01\x80\x86\x01\x82\x90U\x84\x82R`\x02` R`@\x82 \x82\x81U\x92\x93Pa\x06\xF8\x90\x83\x01\x82a\x15 V[PP\x80\x15a\x07=W\x82T`\xFF\x19\x16`\x02\x17\x83U`@Q\x82\x90\x85\x90\x7F\xBF\x89u\x13\x9A\xEE\x07\x94\xECPWC<4\xFB\x93\x9E\x0FeZ\x87\xB0Q\xE3*:\xAE$\xA6U/N\x90`\0\x90\xA3a\x07kV[`@Q\x82\x90\x85\x90\x7F\x84oK\x93k-|\xCF_\xCB\x9F1z\xB7\x91\xF5\xEC\xE5a\x11\x1E\x890n\x99}\x88\xBB\x84*<S\x90`\0\x90\xA3[PPPPV[a\x07z\x81a\x01\xB8V[a\0\xB1\x815a\x06dV[`\0\x81\x81R` \x81\x90R`@\x90 `\x02\x81T`\xFF\x16`\x03\x81\x11\x15a\x07\xAAWa\x07\xAAa\x17\x03V[\x03a\x07\xF3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01Rx\x12[\x9D\x19[\x9D\x08\x1A\\\xC8\x18[\x1C\x99XY\x1EH\x1C\xD9]\x1D\x1B\x19Y`:\x1B`D\x82\x01R`d\x01a\x02\x16V[`\x03\x81T`\xFF\x16`\x03\x81\x11\x15a\x08\x0BWa\x08\x0Ba\x17\x03V[\x03a\x08XW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FIntent is already cancelled\0\0\0\0\0`D\x82\x01R`d\x01a\x02\x16V[\x80T`\xFF\x19\x16`\x03\x17\x81U`@Q\x82\x90\x7F\xC0\x8E\xB6M\xB1j9\xD2\x84\x89`\xAF\x04\xE3\xF1o\xB4\x04\xD9\xD46\xA9\xF0\xE9\xD7\xD0\xD4\x85G\x15\xC9\xDC\x90`\0\x90\xA2PPV[`\x02` R`\0\x90\x81R`@\x90 \x80T`\x01\x82\x01\x80T\x91\x92\x91a\x05\xE1\x90a\x193V[`@\x80Q\x7F\xC2\xF8xqv\xB8\xACk\xF7![J\xDC\xC1\xE0i\xBFJ\xB8-\x9A\xB1\xDF\x05\xA5z\x91\xD4%\x93[n` \x80\x83\x01\x91\x90\x91R\x7F\x97\xBC\x18\x0B\x88[\xB1\x106\xFACp\xBB\x14Lj\xE5U\x8B\xEE\x80\xA1\xF0\x8B\xEF\x9C\xEFq\xAE \x04\x8A\x82\x84\x01R\x7F\x06\xC0\x15\xBD\"\xB4\xC6\x96\x90\x93<\x10X\x87\x8E\xBD\xFE\xF3\x1F\x9A\xAA\xE4\x0B\xBE\x86\xD8\xA0\x9F\xE1\xB2\x97,``\x80\x84\x01\x91\x90\x91RF`\x80\x80\x85\x01\x91\x90\x91R\x84Q\x80\x85\x03\x82\x01\x81R`\xA0\x85\x01\x86R\x80Q\x90\x84\x01 \x86Q\x84\x88\x01Q\x87\x89\x01Q\x94\x89\x01Q\x93\x89\x01Q\x7Fs0\xFFg\xE0\x989\xF9\x8D\xC5\x8Cj\x16\xA1J\xFD[\x92\xEF\xAC\x0C\xD0\xFAv\x05\x92E(zX2\x93`\xC0\x89\x01R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\xE0\x89\x01R\x90\x82\x16a\x01\0\x88\x01Ra\x01 \x87\x01\x94\x90\x94Ra\x01@\x86\x01\x92\x90\x92R\x91\x16a\x01`\x80\x85\x01\x91\x90\x91R\x84Q\x80\x85\x03\x90\x91\x01\x81Ra\x01\x80\x84\x01\x85R\x80Q\x90\x83\x01 a\x19\x01`\xF0\x1Ba\x01\xA0\x85\x01Ra\x01\xA2\x84\x01\x91\x90\x91Ra\x01\xC2\x80\x84\x01\x91\x90\x91R\x83Q\x80\x84\x03\x90\x91\x01\x81Ra\x01\xE2\x90\x92\x01\x90\x92R\x80Q\x91\x01 a\n\x1C\x83\x83`\0\x01Q\x83a\x10\x9DV[PPPV[`\0\x81`\0\x01Q\x82` \x01Q`@Q` \x01a\n>\x92\x91\x90a\x1DQV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`\0\x81`\0\x01Q\x82` \x01Q`@Q` \x01a\n>\x92\x91\x90a\x1DwV[`\0\x81\x81R`\x01` R`@\x81 \x80T\x90\x91\x90\x82\x90a\n\x96\x90a\x193V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\n\xC2\x90a\x193V[\x80\x15a\x0B\x0FW\x80`\x1F\x10a\n\xE4Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0B\x0FV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\n\xF2W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x80` \x01\x90Q\x81\x01\x90a\x0B'\x91\x90a\x1D\xA6V[\x90Pa\n\x1C\x82`\x01\x01\x80Ta\x0B;\x90a\x193V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0Bg\x90a\x193V[\x80\x15a\x0B\xB4W\x80`\x1F\x10a\x0B\x89Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0B\xB4V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0B\x97W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x82a\x08\xB4V[`\0\x81\x81R`\x01` R`@\x81 \x80T\x90\x91\x90\x82\x90a\x0B\xDD\x90a\x193V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0C\t\x90a\x193V[\x80\x15a\x0CVW\x80`\x1F\x10a\x0C+Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0CVV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0C9W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x80` \x01\x90Q\x81\x01\x90a\x0Cn\x91\x90a\x1D\xA6V[\x90Pa\n\x1C\x81` \x01Q\x82`@\x01Q\x83`\0\x01Qa\x11\x87V[`\0\x82\x81R`\x01` R`@\x81 \x80T\x82\x90\x82\x90a\x0C\xA4\x90a\x193V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0C\xD0\x90a\x193V[\x80\x15a\r\x1DW\x80`\x1F\x10a\x0C\xF2Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\r\x1DV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\r\0W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x80` \x01\x90Q\x81\x01\x90a\r5\x91\x90a\x1D\xA6V[\x90P`\0`\x02`\0\x86\x81R` \x01\x90\x81R` \x01`\0 `@Q\x80`@\x01`@R\x90\x81`\0\x82\x01T\x81R` \x01`\x01\x82\x01\x80Ta\rq\x90a\x193V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\r\x9D\x90a\x193V[\x80\x15a\r\xEAW\x80`\x1F\x10a\r\xBFWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\r\xEAV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\r\xCDW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P`\0\x81` \x01Q\x80` \x01\x90Q\x81\x01\x90a\x0E\x0F\x91\x90a\x1E\x10V[\x90P`\0\x83` \x01Q`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0EUW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Ey\x91\x90a\x1EJV[`\xFF\x16\x90P`\0\x84`\x80\x01Q`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E\xC2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\xE6\x91\x90a\x1EJV[`@\x86\x01Q``\x87\x01Q`\xFF\x92\x90\x92\x16\x92P\x90`\0g\r\xE0\xB6\xB3\xA7d\0\0a\x0F\x0F\x85`\na\x1FnV[a\x0F\x19\x90\x84a\x1FzV[a\x0F#\x91\x90a\x1F\x91V[\x90P\x87`\x80\x01Q`\x01`\x01`\xA0\x1B\x03\x16c#\xB8r\xDD3\x8A`\0\x01Q\x88`\na\x0FK\x91\x90a\x1FnV[a\x0FU\x86\x89a\x1FzV[a\x0F_\x91\x90a\x1F\x91V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x86\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16`\x04\x82\x01R\x92\x90\x91\x16`$\x83\x01R`D\x82\x01R`d\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0F\xB3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xD7\x91\x90a\x1F\xB3V[Pa\x0F\xE7\x88` \x01Q\x843a\x11\x93V[\x85` \x01Q\x88`@\x01\x81\x81Qa\x0F\xFD\x91\x90a\x1F\xD5V[\x90RP`@\x88\x01Q\x15\x80\x15a\x10DW`@Q\x8D\x81R\x7FSI^\x046\xD7\x9Dn4\xA3\x03\x0E\xE9\xEB\xE4T+\xAC\xD0\xB5\x9D>\xE5\x03\x89\x9F\xAD!K\xCC;Q\x90` \x01`@Q\x80\x91\x03\x90\xA1a\x10\x8BV[\x7FN\x03`\0H<C\xFD\xD2\xFF\x16\x1C\xE1\xD0\r\xD9\x9F*\xC8e@\x9E\xEC\xCDz\xE8\n%\xA1$?Z\x8D\x88` \x01Q`@Qa\x10\x82\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA1[\x99PPPPPPPPPP[\x92\x91PPV[`\0\x80`\0a\x10\xAB\x86a\x11\x9EV[`@\x80Q`\0\x81R` \x81\x01\x80\x83R\x89\x90R`\xFF\x85\x16\x91\x81\x01\x91\x90\x91R``\x81\x01\x83\x90R`\x80\x81\x01\x82\x90R\x92\x95P\x90\x93P\x91P`\x01`\x01`\xA0\x1B\x03\x86\x16\x90`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x11\x11W=`\0\x80>=`\0\xFD[PPP` `@Q\x03Q`\x01`\x01`\xA0\x1B\x03\x16\x14a\x11\x7FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FVerification error: Signer is in`D\x82\x01Rd\x1D\x98[\x1AY`\xDA\x1B`d\x82\x01R`\x84\x01a\x02\x16V[PPPPPPV[a\n\x1C\x83\x820\x85a\x12 V[a\n\x1C\x83\x82\x84a\x12\x8BV[`\0\x80`\0\x83Q`A\x14a\x12\x04W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FThe signature length is not equa`D\x82\x01Rfl to 65`\xC8\x1B`d\x82\x01R`\x84\x01a\x02\x16V[PPP` \x81\x01Q`@\x82\x01Q``\x90\x92\x01Q`\0\x1A\x92\x90\x91\x90V[`@Q`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`$\x83\x01R\x83\x16`D\x82\x01R`d\x81\x01\x82\x90Ra\x07k\x90\x85\x90c#\xB8r\xDD`\xE0\x1B\x90`\x84\x01[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x93\x16\x92\x90\x92\x17\x90\x91Ra\x12\xBBV[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16`$\x82\x01R`D\x81\x01\x82\x90Ra\n\x1C\x90\x84\x90c\xA9\x05\x9C\xBB`\xE0\x1B\x90`d\x01a\x12TV[`\0a\x13\x10\x82`@Q\x80`@\x01`@R\x80` \x81R` \x01\x7FSafeERC20: low-level call failed\x81RP\x85`\x01`\x01`\xA0\x1B\x03\x16a\x13\x90\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x80Q`\0\x14\x80a\x131WP\x80\x80` \x01\x90Q\x81\x01\x90a\x131\x91\x90a\x1F\xB3V[a\n\x1CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01Ri\x1B\xDD\x08\x1C\xDDX\xD8\xD9YY`\xB2\x1B`d\x82\x01R`\x84\x01a\x02\x16V[``a\x13\x9F\x84\x84`\0\x85a\x13\xA7V[\x94\x93PPPPV[``\x82G\x10\x15a\x14\x08W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FAddress: insufficient balance fo`D\x82\x01Re\x1C\x88\x18\xD8[\x1B`\xD2\x1B`d\x82\x01R`\x84\x01a\x02\x16V[`\0\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@Qa\x14$\x91\x90a\x1F\xE8V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x14aW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x14fV[``\x91P[P\x91P\x91Pa\x14w\x87\x83\x83\x87a\x14\x82V[\x97\x96PPPPPPPV[``\x83\x15a\x14\xF1W\x82Q`\0\x03a\x14\xEAW`\x01`\x01`\xA0\x1B\x03\x85\x16;a\x14\xEAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x02\x16V[P\x81a\x13\x9FV[a\x13\x9F\x83\x83\x81Q\x15a\x15\x06W\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02\x16\x91\x90a \x04V[P\x80Ta\x15,\x90a\x193V[`\0\x82U\x80`\x1F\x10a\x15<WPPV[`\x1F\x01` \x90\x04\x90`\0R` `\0 \x90\x81\x01\x90a\0\xB1\x91\x90[\x80\x82\x11\x15a\x15jW`\0\x81U`\x01\x01a\x15VV[P\x90V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x15\xA7Wa\x15\xA7a\x15nV[`@R\x90V[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x15\xA7Wa\x15\xA7a\x15nV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0\xB1W`\0\x80\xFD[`\0`@\x82\x84\x03\x12\x15a\x15\xF7W`\0\x80\xFD[a\x15\xFFa\x15\x84V[\x825a\x16\n\x81a\x15\xD0V[\x81R` \x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x91\x90PV[`\0`@\x82\x84\x03\x12\x15a\x161W`\0\x80\xFD[P\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x16IW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x16`W`\0\x80\xFD[a\x13\x9F\x84\x82\x85\x01a\x16\x1FV[`\0` \x82\x84\x03\x12\x15a\x16~W`\0\x80\xFD[P5\x91\x90PV[`\0[\x83\x81\x10\x15a\x16\xA0W\x81\x81\x01Q\x83\x82\x01R` \x01a\x16\x88V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x16\xC1\x81` \x86\x01` \x86\x01a\x16\x85V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`@\x81R`\0a\x16\xE8`@\x83\x01\x85a\x16\xA9V[\x82\x81\x03` \x84\x01Ra\x16\xFA\x81\x85a\x16\xA9V[\x95\x94PPPPPV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`@\x81\x01`\x04\x84\x10a\x17;WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x92\x81R` \x01R\x90V[\x82\x81R`@` \x82\x01R`\0a\x13\x9F`@\x83\x01\x84a\x16\xA9V[`\0\x82`\x1F\x83\x01\x12a\x17oW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x17\x8AWa\x17\x8Aa\x15nV[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x17\xB2Wa\x17\xB2a\x15nV[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\x17\xCBW`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[`\0\x80\x82\x84\x03`\xC0\x81\x12\x15a\x17\xFFW`\0\x80\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x18\x16W`\0\x80\xFD[a\x18\"\x86\x82\x87\x01a\x17^V[\x93PP`\xA0`\x1F\x19\x82\x01\x12\x15a\x187W`\0\x80\xFD[Pa\x18@a\x15\xADV[` \x84\x015a\x18N\x81a\x15\xD0V[\x81R`@\x84\x015a\x18^\x81a\x15\xD0V[` \x82\x01R``\x84\x81\x015`@\x83\x01R`\x80\x85\x015\x90\x82\x01R`\xA0\x84\x015a\x18\x85\x81a\x15\xD0V[`\x80\x82\x01R\x91\x94\x91\x93P\x90\x91PPV[`\0`@\x826\x03\x12\x15a\x18\xA7W`\0\x80\xFD[a\x18\xAFa\x15\x84V[\x825\x81R` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x18\xCDW`\0\x80\xFD[a\x18\xD96\x82\x86\x01a\x17^V[` \x83\x01RP\x92\x91PPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x18\xFCW`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x19\x17W`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x19,W`\0\x80\xFD[\x92P\x92\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x19GW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x161WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[`\x1F\x82\x11\x15a\n\x1CW`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a\x19\x8EWP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x11\x7FW\x82\x81U`\x01\x01a\x19\x9AV[`\0\x19`\x03\x83\x90\x1B\x1C\x19\x16`\x01\x91\x90\x91\x1B\x17\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x15a\x19\xDAWa\x19\xDAa\x15nV[a\x19\xEE\x83a\x19\xE8\x83Ta\x193V[\x83a\x19gV[`\0`\x1F\x84\x11`\x01\x81\x14a\x1A\x1CW`\0\x85\x15a\x1A\nWP\x83\x82\x015[a\x1A\x14\x86\x82a\x19\xADV[\x84UPa\x1AvV[`\0\x83\x81R` \x90 `\x1F\x19\x86\x16\x90\x83[\x82\x81\x10\x15a\x1AMW\x86\x85\x015\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a\x1A-V[P\x86\x82\x10\x15a\x1AjW`\0\x19`\xF8\x88`\x03\x1B\x16\x1C\x19\x84\x87\x015\x16\x81U[PP`\x01\x85`\x01\x1B\x01\x83U[PPPPPV[\x815\x81U`\x01\x80\x82\x01` a\x1A\x94\x81\x86\x01\x86a\x18\xE5V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1A\xACWa\x1A\xACa\x15nV[a\x1A\xC0\x81a\x1A\xBA\x86Ta\x193V[\x86a\x19gV[`\0`\x1F\x82\x11`\x01\x81\x14a\x1A\xEEW`\0\x83\x15a\x1A\xDCWP\x83\x82\x015[a\x1A\xE6\x84\x82a\x19\xADV[\x87UPa\x1BCV[`\0\x86\x81R` \x90 `\x1F\x19\x84\x16\x90\x83[\x82\x81\x10\x15a\x1B\x1CW\x86\x85\x015\x82U\x93\x87\x01\x93\x90\x89\x01\x90\x87\x01a\x1A\xFFV[P\x84\x82\x10\x15a\x1B9W`\0\x19`\xF8\x86`\x03\x1B\x16\x1C\x19\x84\x87\x015\x16\x81U[PP\x86\x83\x88\x1B\x01\x86U[PPPPPPPPPV[`\0`@\x826\x03\x12\x15a\x1B`W`\0\x80\xFD[a\x1Bha\x15\x84V[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x1B\x80W`\0\x80\xFD[a\x1B\x8C6\x83\x87\x01a\x17^V[\x83R` \x85\x015\x91P\x80\x82\x11\x15a\x1B\xA2W`\0\x80\xFD[Pa\x18\xD96\x82\x86\x01a\x17^V[a\x1B\xB9\x82\x83a\x18\xE5V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1B\xD1Wa\x1B\xD1a\x15nV[a\x1B\xE5\x81a\x1B\xDF\x85Ta\x193V[\x85a\x19gV[`\0`\x1F\x82\x11`\x01\x81\x14a\x1C\x13W`\0\x83\x15a\x1C\x01WP\x83\x82\x015[a\x1C\x0B\x84\x82a\x19\xADV[\x86UPa\x1CmV[`\0\x85\x81R` \x90 `\x1F\x19\x84\x16\x90\x83[\x82\x81\x10\x15a\x1CDW\x86\x85\x015\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a\x1C$V[P\x84\x82\x10\x15a\x1CaW`\0\x19`\xF8\x86`\x03\x1B\x16\x1C\x19\x84\x87\x015\x16\x81U[PP`\x01\x83`\x01\x1B\x01\x85U[PPPPa\x1C~` \x83\x01\x83a\x18\xE5V[a\x07k\x81\x83`\x01\x86\x01a\x19\xC2V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x1C\xA3W`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1C\xC3W`\0\x80\xFD[\x806\x03\x82\x13\x15a\x19,W`\0\x80\xFD[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[` \x81R`\0a\x1D\x0B\x83\x84a\x1C\x8CV[`@` \x85\x01Ra\x1D ``\x85\x01\x82\x84a\x1C\xD2V[\x91PPa\x1D0` \x85\x01\x85a\x1C\x8CV[\x84\x83\x03`\x1F\x19\x01`@\x86\x01Ra\x1DG\x83\x82\x84a\x1C\xD2V[\x96\x95PPPPPPV[\x82\x81R`\0\x82Qa\x1Di\x81` \x85\x01` \x87\x01a\x16\x85V[\x91\x90\x91\x01` \x01\x93\x92PPPV[`\0\x83Qa\x1D\x89\x81\x84` \x88\x01a\x16\x85V[\x83Q\x90\x83\x01\x90a\x1D\x9D\x81\x83` \x88\x01a\x16\x85V[\x01\x94\x93PPPPV[`\0`\xA0\x82\x84\x03\x12\x15a\x1D\xB8W`\0\x80\xFD[a\x1D\xC0a\x15\xADV[\x82Qa\x1D\xCB\x81a\x15\xD0V[\x81R` \x83\x01Qa\x1D\xDB\x81a\x15\xD0V[\x80` \x83\x01RP`@\x83\x01Q`@\x82\x01R``\x83\x01Q``\x82\x01R`\x80\x83\x01Qa\x1E\x04\x81a\x15\xD0V[`\x80\x82\x01R\x93\x92PPPV[`\0`@\x82\x84\x03\x12\x15a\x1E\"W`\0\x80\xFD[a\x1E*a\x15\x84V[\x82Qa\x1E5\x81a\x15\xD0V[\x81R` \x92\x83\x01Q\x92\x81\x01\x92\x90\x92RP\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x1E\\W`\0\x80\xFD[\x81Q`\xFF\x81\x16\x81\x14a\x1EmW`\0\x80\xFD[\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\x01\x81\x81[\x80\x85\x11\x15a\x1E\xC5W\x81`\0\x19\x04\x82\x11\x15a\x1E\xABWa\x1E\xABa\x1EtV[\x80\x85\x16\x15a\x1E\xB8W\x91\x81\x02\x91[\x93\x84\x1C\x93\x90\x80\x02\x90a\x1E\x8FV[P\x92P\x92\x90PV[`\0\x82a\x1E\xDCWP`\x01a\x10\x97V[\x81a\x1E\xE9WP`\0a\x10\x97V[\x81`\x01\x81\x14a\x1E\xFFW`\x02\x81\x14a\x1F\tWa\x1F%V[`\x01\x91PPa\x10\x97V[`\xFF\x84\x11\x15a\x1F\x1AWa\x1F\x1Aa\x1EtV[PP`\x01\x82\x1Ba\x10\x97V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a\x1FHWP\x81\x81\na\x10\x97V[a\x1FR\x83\x83a\x1E\x8AV[\x80`\0\x19\x04\x82\x11\x15a\x1FfWa\x1Ffa\x1EtV[\x02\x93\x92PPPV[`\0a\x1Em\x83\x83a\x1E\xCDV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x10\x97Wa\x10\x97a\x1EtV[`\0\x82a\x1F\xAEWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\0` \x82\x84\x03\x12\x15a\x1F\xC5W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x1EmW`\0\x80\xFD[\x81\x81\x03\x81\x81\x11\x15a\x10\x97Wa\x10\x97a\x1EtV[`\0\x82Qa\x1F\xFA\x81\x84` \x87\x01a\x16\x85V[\x91\x90\x91\x01\x92\x91PPV[` \x81R`\0a\x1Em` \x83\x01\x84a\x16\xA9V\xFE\xA2dipfsX\"\x12 \xE9\xA5\xB1m\xA8\t\x18\xE4\x96\x13)*\xE7\xC2\xAEm\\\x7F\xA8\xE7Mj\xB1\xF1@\xA2C \xAB\xB6\x0F\xDDdsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static LIMITORDERINTENTBOOK_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct LimitOrderIntentBook<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for LimitOrderIntentBook<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for LimitOrderIntentBook<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for LimitOrderIntentBook<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for LimitOrderIntentBook<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(LimitOrderIntentBook))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> LimitOrderIntentBook<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    LIMITORDERINTENTBOOK_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                LIMITORDERINTENTBOOK_ABI.clone(),
                LIMITORDERINTENTBOOK_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `cancelIntent` (0xd55f960d) function
        pub fn cancel_intent(
            &self,
            intent_id: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([213, 95, 150, 13], intent_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `intentBidData` (0xe25623e0) function
        pub fn intent_bid_data(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ([u8; 32], ::ethers::core::types::Bytes),
        > {
            self.0
                .method_hash([226, 86, 35, 224], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `intentData` (0x59a844b4) function
        pub fn intent_data(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::Bytes, ::ethers::core::types::Bytes),
        > {
            self.0
                .method_hash([89, 168, 68, 180], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `intentStates` (0x5ff8a66b) function
        pub fn intent_states(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, (u8, [u8; 32])> {
            self.0
                .method_hash([95, 248, 166, 107], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `matchAndSettle` (0x87f617b6) function
        pub fn match_and_settle(
            &self,
            intent_bid: IntentBid,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([135, 246, 23, 182], (intent_bid,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `matchIntent` (0x09c7b2f6) function
        pub fn match_intent(
            &self,
            intent_bid: IntentBid,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([9, 199, 178, 246], (intent_bid,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `placeIntent` (0x4af2364e) function
        pub fn place_intent(
            &self,
            intent: Intent,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([74, 242, 54, 78], (intent,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `settleIntent` (0x7bf8bb88) function
        pub fn settle_intent(
            &self,
            intent_id: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([123, 248, 187, 136], intent_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `verify` (0x03895c91) function
        pub fn verify(
            &self,
            limit_order_bid: LimitOrderBid,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([3, 137, 92, 145], (limit_order_bid,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `verifySignature` (0xee5701e7) function
        pub fn verify_signature(
            &self,
            signature: ::ethers::core::types::Bytes,
            limit_order: LimitOrder,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([238, 87, 1, 231], (signature, limit_order))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `IntentBidReceived` event
        pub fn intent_bid_received_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            IntentBidReceivedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `IntentCancelled` event
        pub fn intent_cancelled_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            IntentCancelledFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `IntentCreated` event
        pub fn intent_created_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            IntentCreatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `IntentMatch` event
        pub fn intent_match_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            IntentMatchFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `IntentPartiallySettled` event
        pub fn intent_partially_settled_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            IntentPartiallySettledFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `IntentSettled` event
        pub fn intent_settled_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            IntentSettledFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `LimitOrderFulfilled` event
        pub fn limit_order_fulfilled_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LimitOrderFulfilledFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `LimitOrderPartialFill` event
        pub fn limit_order_partial_fill_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LimitOrderPartialFillFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LimitOrderIntentBookEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for LimitOrderIntentBook<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `NotImplemented` with signature `NotImplemented()` and selector `0xd6234725`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "NotImplemented", abi = "NotImplemented()")]
    pub struct NotImplemented;
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "IntentBidReceived",
        abi = "IntentBidReceived(bytes32,bytes32,(bytes32,bytes))"
    )]
    pub struct IntentBidReceivedFilter {
        #[ethevent(indexed)]
        pub intent_id: [u8; 32],
        #[ethevent(indexed)]
        pub intent_bid_id: [u8; 32],
        pub intent_bid: IntentBid,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "IntentCancelled", abi = "IntentCancelled(bytes32)")]
    pub struct IntentCancelledFilter {
        #[ethevent(indexed)]
        pub intent_id: [u8; 32],
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "IntentCreated", abi = "IntentCreated(bytes32,(bytes,bytes))")]
    pub struct IntentCreatedFilter {
        #[ethevent(indexed)]
        pub intent_id: [u8; 32],
        pub intent: Intent,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "IntentMatch", abi = "IntentMatch(bytes32,bytes32)")]
    pub struct IntentMatchFilter {
        #[ethevent(indexed)]
        pub intent_id: [u8; 32],
        #[ethevent(indexed)]
        pub intent_bid_id: [u8; 32],
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "IntentPartiallySettled",
        abi = "IntentPartiallySettled(bytes32,bytes32)"
    )]
    pub struct IntentPartiallySettledFilter {
        #[ethevent(indexed)]
        pub intent_id: [u8; 32],
        #[ethevent(indexed)]
        pub intent_bid_id: [u8; 32],
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "IntentSettled", abi = "IntentSettled(bytes32,bytes32)")]
    pub struct IntentSettledFilter {
        #[ethevent(indexed)]
        pub intent_id: [u8; 32],
        #[ethevent(indexed)]
        pub intent_bid_id: [u8; 32],
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "LimitOrderFulfilled", abi = "LimitOrderFulfilled(bytes32)")]
    pub struct LimitOrderFulfilledFilter {
        pub intent_id: [u8; 32],
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "LimitOrderPartialFill",
        abi = "LimitOrderPartialFill(bytes32,uint256)"
    )]
    pub struct LimitOrderPartialFillFilter {
        pub intent_id: [u8; 32],
        pub volume_filled: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's events
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub enum LimitOrderIntentBookEvents {
        IntentBidReceivedFilter(IntentBidReceivedFilter),
        IntentCancelledFilter(IntentCancelledFilter),
        IntentCreatedFilter(IntentCreatedFilter),
        IntentMatchFilter(IntentMatchFilter),
        IntentPartiallySettledFilter(IntentPartiallySettledFilter),
        IntentSettledFilter(IntentSettledFilter),
        LimitOrderFulfilledFilter(LimitOrderFulfilledFilter),
        LimitOrderPartialFillFilter(LimitOrderPartialFillFilter),
    }
    impl ::ethers::contract::EthLogDecode for LimitOrderIntentBookEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = IntentBidReceivedFilter::decode_log(log) {
                return Ok(LimitOrderIntentBookEvents::IntentBidReceivedFilter(decoded));
            }
            if let Ok(decoded) = IntentCancelledFilter::decode_log(log) {
                return Ok(LimitOrderIntentBookEvents::IntentCancelledFilter(decoded));
            }
            if let Ok(decoded) = IntentCreatedFilter::decode_log(log) {
                return Ok(LimitOrderIntentBookEvents::IntentCreatedFilter(decoded));
            }
            if let Ok(decoded) = IntentMatchFilter::decode_log(log) {
                return Ok(LimitOrderIntentBookEvents::IntentMatchFilter(decoded));
            }
            if let Ok(decoded) = IntentPartiallySettledFilter::decode_log(log) {
                return Ok(
                    LimitOrderIntentBookEvents::IntentPartiallySettledFilter(decoded),
                );
            }
            if let Ok(decoded) = IntentSettledFilter::decode_log(log) {
                return Ok(LimitOrderIntentBookEvents::IntentSettledFilter(decoded));
            }
            if let Ok(decoded) = LimitOrderFulfilledFilter::decode_log(log) {
                return Ok(
                    LimitOrderIntentBookEvents::LimitOrderFulfilledFilter(decoded),
                );
            }
            if let Ok(decoded) = LimitOrderPartialFillFilter::decode_log(log) {
                return Ok(
                    LimitOrderIntentBookEvents::LimitOrderPartialFillFilter(decoded),
                );
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for LimitOrderIntentBookEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::IntentBidReceivedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IntentCancelledFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IntentCreatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IntentMatchFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::IntentPartiallySettledFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IntentSettledFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LimitOrderFulfilledFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LimitOrderPartialFillFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<IntentBidReceivedFilter> for LimitOrderIntentBookEvents {
        fn from(value: IntentBidReceivedFilter) -> Self {
            Self::IntentBidReceivedFilter(value)
        }
    }
    impl ::core::convert::From<IntentCancelledFilter> for LimitOrderIntentBookEvents {
        fn from(value: IntentCancelledFilter) -> Self {
            Self::IntentCancelledFilter(value)
        }
    }
    impl ::core::convert::From<IntentCreatedFilter> for LimitOrderIntentBookEvents {
        fn from(value: IntentCreatedFilter) -> Self {
            Self::IntentCreatedFilter(value)
        }
    }
    impl ::core::convert::From<IntentMatchFilter> for LimitOrderIntentBookEvents {
        fn from(value: IntentMatchFilter) -> Self {
            Self::IntentMatchFilter(value)
        }
    }
    impl ::core::convert::From<IntentPartiallySettledFilter>
    for LimitOrderIntentBookEvents {
        fn from(value: IntentPartiallySettledFilter) -> Self {
            Self::IntentPartiallySettledFilter(value)
        }
    }
    impl ::core::convert::From<IntentSettledFilter> for LimitOrderIntentBookEvents {
        fn from(value: IntentSettledFilter) -> Self {
            Self::IntentSettledFilter(value)
        }
    }
    impl ::core::convert::From<LimitOrderFulfilledFilter>
    for LimitOrderIntentBookEvents {
        fn from(value: LimitOrderFulfilledFilter) -> Self {
            Self::LimitOrderFulfilledFilter(value)
        }
    }
    impl ::core::convert::From<LimitOrderPartialFillFilter>
    for LimitOrderIntentBookEvents {
        fn from(value: LimitOrderPartialFillFilter) -> Self {
            Self::LimitOrderPartialFillFilter(value)
        }
    }
    ///Container type for all input parameters for the `cancelIntent` function with signature `cancelIntent(bytes32)` and selector `0xd55f960d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "cancelIntent", abi = "cancelIntent(bytes32)")]
    pub struct CancelIntentCall {
        pub intent_id: [u8; 32],
    }
    ///Container type for all input parameters for the `intentBidData` function with signature `intentBidData(bytes32)` and selector `0xe25623e0`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "intentBidData", abi = "intentBidData(bytes32)")]
    pub struct IntentBidDataCall(pub [u8; 32]);
    ///Container type for all input parameters for the `intentData` function with signature `intentData(bytes32)` and selector `0x59a844b4`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "intentData", abi = "intentData(bytes32)")]
    pub struct IntentDataCall(pub [u8; 32]);
    ///Container type for all input parameters for the `intentStates` function with signature `intentStates(bytes32)` and selector `0x5ff8a66b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "intentStates", abi = "intentStates(bytes32)")]
    pub struct IntentStatesCall(pub [u8; 32]);
    ///Container type for all input parameters for the `matchAndSettle` function with signature `matchAndSettle((bytes32,bytes))` and selector `0x87f617b6`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "matchAndSettle", abi = "matchAndSettle((bytes32,bytes))")]
    pub struct MatchAndSettleCall {
        pub intent_bid: IntentBid,
    }
    ///Container type for all input parameters for the `matchIntent` function with signature `matchIntent((bytes32,bytes))` and selector `0x09c7b2f6`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "matchIntent", abi = "matchIntent((bytes32,bytes))")]
    pub struct MatchIntentCall {
        pub intent_bid: IntentBid,
    }
    ///Container type for all input parameters for the `placeIntent` function with signature `placeIntent((bytes,bytes))` and selector `0x4af2364e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "placeIntent", abi = "placeIntent((bytes,bytes))")]
    pub struct PlaceIntentCall {
        pub intent: Intent,
    }
    ///Container type for all input parameters for the `settleIntent` function with signature `settleIntent(bytes32)` and selector `0x7bf8bb88`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "settleIntent", abi = "settleIntent(bytes32)")]
    pub struct SettleIntentCall {
        pub intent_id: [u8; 32],
    }
    ///Container type for all input parameters for the `verify` function with signature `verify((address,uint256))` and selector `0x03895c91`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "verify", abi = "verify((address,uint256))")]
    pub struct VerifyCall {
        pub limit_order_bid: LimitOrderBid,
    }
    ///Container type for all input parameters for the `verifySignature` function with signature `verifySignature(bytes,(address,address,uint256,uint256,address))` and selector `0xee5701e7`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "verifySignature",
        abi = "verifySignature(bytes,(address,address,uint256,uint256,address))"
    )]
    pub struct VerifySignatureCall {
        pub signature: ::ethers::core::types::Bytes,
        pub limit_order: LimitOrder,
    }
    ///Container type for all of the contract's call
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub enum LimitOrderIntentBookCalls {
        CancelIntent(CancelIntentCall),
        IntentBidData(IntentBidDataCall),
        IntentData(IntentDataCall),
        IntentStates(IntentStatesCall),
        MatchAndSettle(MatchAndSettleCall),
        MatchIntent(MatchIntentCall),
        PlaceIntent(PlaceIntentCall),
        SettleIntent(SettleIntentCall),
        Verify(VerifyCall),
        VerifySignature(VerifySignatureCall),
    }
    impl ::ethers::core::abi::AbiDecode for LimitOrderIntentBookCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <CancelIntentCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CancelIntent(decoded));
            }
            if let Ok(decoded) = <IntentBidDataCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IntentBidData(decoded));
            }
            if let Ok(decoded) = <IntentDataCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IntentData(decoded));
            }
            if let Ok(decoded) = <IntentStatesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IntentStates(decoded));
            }
            if let Ok(decoded) = <MatchAndSettleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MatchAndSettle(decoded));
            }
            if let Ok(decoded) = <MatchIntentCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MatchIntent(decoded));
            }
            if let Ok(decoded) = <PlaceIntentCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PlaceIntent(decoded));
            }
            if let Ok(decoded) = <SettleIntentCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SettleIntent(decoded));
            }
            if let Ok(decoded) = <VerifyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Verify(decoded));
            }
            if let Ok(decoded) = <VerifySignatureCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::VerifySignature(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for LimitOrderIntentBookCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::CancelIntent(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IntentBidData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IntentData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IntentStates(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MatchAndSettle(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MatchIntent(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PlaceIntent(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SettleIntent(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Verify(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::VerifySignature(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for LimitOrderIntentBookCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CancelIntent(element) => ::core::fmt::Display::fmt(element, f),
                Self::IntentBidData(element) => ::core::fmt::Display::fmt(element, f),
                Self::IntentData(element) => ::core::fmt::Display::fmt(element, f),
                Self::IntentStates(element) => ::core::fmt::Display::fmt(element, f),
                Self::MatchAndSettle(element) => ::core::fmt::Display::fmt(element, f),
                Self::MatchIntent(element) => ::core::fmt::Display::fmt(element, f),
                Self::PlaceIntent(element) => ::core::fmt::Display::fmt(element, f),
                Self::SettleIntent(element) => ::core::fmt::Display::fmt(element, f),
                Self::Verify(element) => ::core::fmt::Display::fmt(element, f),
                Self::VerifySignature(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CancelIntentCall> for LimitOrderIntentBookCalls {
        fn from(value: CancelIntentCall) -> Self {
            Self::CancelIntent(value)
        }
    }
    impl ::core::convert::From<IntentBidDataCall> for LimitOrderIntentBookCalls {
        fn from(value: IntentBidDataCall) -> Self {
            Self::IntentBidData(value)
        }
    }
    impl ::core::convert::From<IntentDataCall> for LimitOrderIntentBookCalls {
        fn from(value: IntentDataCall) -> Self {
            Self::IntentData(value)
        }
    }
    impl ::core::convert::From<IntentStatesCall> for LimitOrderIntentBookCalls {
        fn from(value: IntentStatesCall) -> Self {
            Self::IntentStates(value)
        }
    }
    impl ::core::convert::From<MatchAndSettleCall> for LimitOrderIntentBookCalls {
        fn from(value: MatchAndSettleCall) -> Self {
            Self::MatchAndSettle(value)
        }
    }
    impl ::core::convert::From<MatchIntentCall> for LimitOrderIntentBookCalls {
        fn from(value: MatchIntentCall) -> Self {
            Self::MatchIntent(value)
        }
    }
    impl ::core::convert::From<PlaceIntentCall> for LimitOrderIntentBookCalls {
        fn from(value: PlaceIntentCall) -> Self {
            Self::PlaceIntent(value)
        }
    }
    impl ::core::convert::From<SettleIntentCall> for LimitOrderIntentBookCalls {
        fn from(value: SettleIntentCall) -> Self {
            Self::SettleIntent(value)
        }
    }
    impl ::core::convert::From<VerifyCall> for LimitOrderIntentBookCalls {
        fn from(value: VerifyCall) -> Self {
            Self::Verify(value)
        }
    }
    impl ::core::convert::From<VerifySignatureCall> for LimitOrderIntentBookCalls {
        fn from(value: VerifySignatureCall) -> Self {
            Self::VerifySignature(value)
        }
    }
    ///Container type for all return fields from the `intentBidData` function with signature `intentBidData(bytes32)` and selector `0xe25623e0`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct IntentBidDataReturn {
        pub intent_id: [u8; 32],
        pub bid: ::ethers::core::types::Bytes,
    }
    ///Container type for all return fields from the `intentData` function with signature `intentData(bytes32)` and selector `0x59a844b4`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct IntentDataReturn {
        pub intent: ::ethers::core::types::Bytes,
        pub signature: ::ethers::core::types::Bytes,
    }
    ///Container type for all return fields from the `intentStates` function with signature `intentStates(bytes32)` and selector `0x5ff8a66b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct IntentStatesReturn {
        pub status: u8,
        pub intent_bid_id: [u8; 32],
    }
    ///Container type for all return fields from the `placeIntent` function with signature `placeIntent((bytes,bytes))` and selector `0x4af2364e`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct PlaceIntentReturn {
        pub intent_id: [u8; 32],
    }
    ///`LimitOrder(address,address,uint256,uint256,address)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct LimitOrder {
        pub author: ::ethers::core::types::Address,
        pub token: ::ethers::core::types::Address,
        pub volume: ::ethers::core::types::U256,
        pub price: ::ethers::core::types::U256,
        pub out_token: ::ethers::core::types::Address,
    }
    ///`LimitOrderBid(address,uint256)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct LimitOrderBid {
        pub filler: ::ethers::core::types::Address,
        pub volume: ::ethers::core::types::U256,
    }
}
