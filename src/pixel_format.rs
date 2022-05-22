/*
 * Copyright 2022 l1npengtul <l1npengtul@protonmail.com> / The Nokhwa Contributors
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use crate::buffer_output::{BufferOutput, GrayU8, RgbU8};
use crate::FrameFormat;
use image::{Luma, Pixel, Rgb};
use std::fmt::Debug;
use std::hash::Hash;

pub trait PixelFormat: Copy + Clone + Debug + Default + Hash + Send + Sync {
    type Output: Pixel;

    const SUPPORTED_CODES: &'static [FrameFormat];
}