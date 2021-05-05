// Copyright 2019-present, the HuggingFace Inc. team, The Google AI Language Team and Facebook, Inc.
// Copyright 2019 Guillaume Becquin
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//     http://www.apache.org/licenses/LICENSE-2.0
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate anyhow;

use rust_bert::pipelines::summarization::SummarizationModel;
use rust_bert::pipelines::translation::{Language, TranslationConfig, TranslationModel};
use std::io::Read;

use tch::Device;

fn main() -> anyhow::Result<()> {
    let summarization_model = SummarizationModel::new(Default::default())?;

    let mut file = std::fs::File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("Content:");
    print!("{}", contents);
    let translation_config =
        TranslationConfig::new(Language::EnglishToGerman, Device::cuda_if_available());
    let model = TranslationModel::new(translation_config)?;

    //    Credits: WikiNews, CC BY 2.5 license (https://en.wikinews.org/wiki/Astronomers_find_water_vapour_in_atmosphere_of_exoplanet_K2-18b)
    let summary = summarization_model.summarize(&[contents.as_ref()]);
    for sentence in summary {
        println!("");
        println!("Summary:");
        println!("{:?}", sentence);
        println!("Translate:");

        let output = model.translate(&[sentence.as_ref()]);

        for sentence in output {
            println!("{}", sentence);
        }
    }

    Ok(())
}
