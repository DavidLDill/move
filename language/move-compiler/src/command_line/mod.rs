// Copyright (c) The Diem Core Contributors
// SPDX-License-Identifier: Apache-2.0

pub mod compiler;

pub const DEPENDENCY: &str = "dependency";
pub const DEPENDENCY_SHORT: char = 'd';

pub const SENDER: &str = "sender";
pub const SENDER_SHORT: char = 's';

pub const OUT_DIR: &str = "out-dir";
pub const OUT_DIR_SHORT: char = 'o';
pub const DEFAULT_OUTPUT_DIR: &str = "build";

pub const NO_SHADOW: &str = "no-shadow";
pub const NO_SHADOW_SHORT: char = 'S';

pub const SOURCE_MAP: &str = "source-map";
pub const SOURCE_MAP_SHORT: char = 'm';

pub const TEST: &str = "test";
pub const TEST_SHORT: char = 't';

pub const FLAVOR: &str = "flavor";

pub const COLOR_MODE_ENV_VAR: &str = "COLOR_MODE";

pub const MOVE_COMPILED_INTERFACES_DIR: &str = "mv_interfaces";

pub const COMPILED_NAMED_ADDRESS_MAPPING: &str = "compiled-module-address-name";
