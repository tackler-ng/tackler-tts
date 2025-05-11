/*
 * Tackler-TTS 2025
 * SPDX-License-Identifier: Apache-2.0
 */

use tackler_tts_core::add as core_add;
use tackler_tts_git::add as git_add;

fn main() {
    println!("Hello, world! {}", core_add(3, 4));
    println!("Hello, world! {}", git_add(3, 4));
}
