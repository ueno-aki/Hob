#[cfg(test)]
mod tests {
    #![allow(non_snake_case, dead_code)]
    use serde::Deserialize;
    #[test]
    fn get_leveldb_works() {
        let vec = Vec::from_iter(include_bytes!("../level.dat").to_vec().into_iter().skip(8));
        let level: LevelDB = serde_nbt::LittleEndian::from_buffer(&vec).unwrap();

        assert_eq!(
            level,
            LevelDB {
                LevelName: String::from("マイ ワールド"),
                FlatWorldLayers: String::from("{\"biome_id\":1,\"block_layers\":[{\"block_name\":\"minecraft:bedrock\",\"count\":1},{\"block_name\":\"minecraft:dirt\",\"count\":2},{\"block_name\":\"minecraft:grass\",\"count\":1}],\"encoding_version\":6,\"structure_options\":null,\"world_version\":\"version.post_1_18\"}\n"),
                abilities: Abilities {
                    mayfly: 0,
                    mine: 1,
                    op: 0,
                    opencontainers: 1,
                    teleport: 0,
                    walkSpeed: 0.1
                },
                SpawnX: 12,
                SpawnY: 32767,
                SpawnZ: 41,
                experiments: Experiments { experiments_ever_used: 0, saved_with_toggled_experiments: 0 },
                lastOpenedWithVersion: vec![1, 20, 40, 1, 0],
                worldStartCount: 4294967294
            }
        )
    }

    #[derive(Debug, Deserialize, PartialEq)]
    struct LevelDB {
        LevelName: String,
        FlatWorldLayers: String,
        abilities: Abilities,
        SpawnX: i32,
        SpawnY: i32,
        SpawnZ: i32,
        experiments: Experiments,
        lastOpenedWithVersion: Vec<i32>,
        worldStartCount: i64,
    }
    #[derive(Debug, Deserialize, PartialEq)]
    struct Abilities {
        mayfly: i8,
        mine: i8,
        op: i8,
        opencontainers: i8,
        teleport: i8,
        walkSpeed: f32,
    }
    #[derive(Debug, Deserialize, PartialEq)]
    struct Experiments {
        experiments_ever_used: i8,
        saved_with_toggled_experiments: i8,
    }
}
