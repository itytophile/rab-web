pub trait English {
    fn to_english(&self) -> &'static str;
}

impl English for crate::DisplaySkill {
    fn to_english(&self) -> &'static str {
        use rab_core::armor_and_skills::Skill::*;

        match self.0 {
            AttackBoost => "Attack Boost",
            Agitator => "Agitator",
            PeakPerformance => "Peak Performance",
            Resentment => "Resentment",
            Resuscitate => "Resuscitate",
            CriticalEye => "Critical Eye",
            CriticalBoost => "Critical Boost",
            WeaknessExploit => "Weakness Exploit",
            LatentPower => "Latent Power",
            MaximumMight => "Maximum Might",
            CriticalElement => "Critical Element",
            MastersTouch => "Master's Touch",
            FireAttack => "Fire Attack",
            WaterAttack => "Water Attack",
            IceAttack => "Ice Attack",
            ThunderAttack => "Thunder Attack",
            DragonAttack => "Dragon Attack",
            PoisonAttack => "Poison Attack",
            ParalysisAttack => "Paralysis Attack",
            SleepAttack => "Sleep Attack",
            BlastAttack => "Blast Attack",
            Handicraft => "Handicraft",
            RazorSharp => "Razor Sharp",
            SpareShot => "Spare Shot",
            ProtectivePolish => "Protective Polish",
            MindsEye => "Mind's Eye",
            Ballistics => "Ballistics",
            Bludgeoner => "Bludgeoner",
            BowChargePlus => "Bow Charge Plus",
            Focus => "Focus",
            PowerProlonger => "Power Prolonger",
            MarathonRunner => "Marathon Runner",
            Constitution => "Constitution",
            StaminaSurge => "Stamina Surge",
            Guard => "Guard",
            GuardUp => "Guard Up",
            OffensiveGuard => "Offensive Guard",
            CriticalDraw => "Critical Draw",
            PunishingDraw => "Punishing Draw",
            QuickSheath => "Quick Sheath",
            Slugger => "Slugger",
            StaminaThief => "Stamina Thief",
            AffinitySliding => "Affinity Sliding",
            HornMaestro => "Horn Maestro",
            Artillery => "Artillery",
            LoadShells => "Load Shells",
            SpecialAmmoBoost => "Special Ammo Boost",
            NormalRapidUp => "Normal/Rapid Up",
            PierceUp => "Pierce Up",
            SpreadUp => "Spread Up",
            AmmoUp => "Ammo Up",
            ReloadSpeed => "Reload Speed",
            RecoilDown => "Recoil Down",
            Steadiness => "Steadiness",
            RapidFireUp => "Rapid Fire Up",
            DefenseBoost => "Defense Boost",
            DivineBlessing => "Divine Blessing",
            RecoveryUp => "Recovery Up",
            RecoverySpeed => "Recovery Speed",
            SpeedEating => "Speed Eating",
            Earplugs => "Earplugs",
            Windproof => "Windproof",
            TremorResistance => "Tremor Resistance",
            BubblyDance => "Bubbly Dance",
            EvadeWindow => "Evade Window",
            EvadeExtender => "Evade Extender",
            FireResistance => "Fire Resistance",
            WaterResistance => "Water Resistance",
            IceResistance => "Ice Resistance",
            ThunderResistance => "Thunder Resistance",
            DragonResistance => "Dragon Resistance",
            BlightResistance => "Blight Resistance",
            PoisonResistance => "Poison Resistance",
            ParalysisResistance => "Paralysis Resistance",
            SleepResistance => "Sleep Resistance",
            StunResistance => "Stun Resistance",
            MuckResistance => "Muck Resistance",
            BlastResistance => "Blast Resistance",
            Botanist => "Botanist",
            Geologist => "Geologist",
            Partbreaker => "Partbreaker",
            CaptureMaster => "Capture Master",
            CarvingMaster => "Carving Master",
            GoodLuck => "Good Luck",
            SpeedSharpening => "Speed Sharpening",
            Bombardier => "Bombardier",
            Mushroomancer => "Mushroomancer",
            ItemProlonger => "Item Prolonger",
            WideRange => "Wide-Range",
            FreeMeal => "Free Meal",
            Heroics => "Heroics",
            Fortify => "Fortify",
            FlinchFree => "Flinch Free",
            JumpMaster => "Jump Master",
            CarvingPro => "Carving Pro",
            HungerResistance => "Hunger Resistance",
            LeapofFaith => "Leap of Faith",
            Diversion => "Diversion",
            MasterMounter => "Master Mounter",
            ChameleosBlessing => "Chameleos Blessing",
            KushalaBlessing => "Kushala Blessing",
            TeostraBlessing => "Teostra Blessing",
            WirebugWhisperer => "Wirebug Whisperer",
            WallRunner => "Wall Runner",
            Counterstrike => "Counterstrike",
            RapidMorph => "Rapid Morph",
            HellfireCloak => "Hellfire Cloak",
            WindAlignment => "Wind Alignment",
            ThunderAlignment => "Thunder Alignment",
            Dragonheart => "Dragonheart",
            Stormsoul => "Stormsoul",
        }
    }
}

impl English for super::UiSymbole {
    fn to_english(&self) -> &'static str {
        use super::UiSymbole::*;

        match self {
            AddSkill => "Add skill",
            SearchBuilds => "Search builds",
            Filter => "Filter",
            MyTalismans => "My talismans",
            Home => "Search",
            TalismansName => "Talisman's name",
            NoSlots => "No slots",
            ShowSkills => "Show skills",
            HideSkills => "Hide skills",
            MyBuilds => "My builds",
        }
    }
}

impl English for crate::armors::Armor {
    fn to_english(&self) -> &'static str {
        use crate::armors::Armor::*;

        match self {
            VaikBraces => "Vaik Braces",
            KushalaGrip => "Kushala Grip",
            BnahabraCoil => "Bnahabra Coil",
            RhopessaVertex => "Rhopessa Vertex",
            AnjanathCoil => "Anjanath Coil",
            RhenoplosCoilS => "Rhenoplos Coil S",
            MelahoaJacketS => "Melahoa Jacket S",
            EdelRibplateS => "Edel Ribplate S",
            RathalosHelmS => "Rathalos Helm S",
            ChannelersHope => "Channeler's Hope",
            KadachiBracesS => "Kadachi Braces S",
            RathianHelm => "Rathian Helm",
            MosgharlRibplateS => "Mosgharl Ribplate S",
            IzuchiGreavesS => "Izuchi Greaves S",
            MediumsHakamaS => "Medium's Hakama S",
            MosgharlFoliaS => "Mosgharl Folia S",
            RhopessaElytraS => "Rhopessa Elytra S",
            EdelRootsS => "Edel Roots S",
            SStuddedSashS => "S. Studded Sash S",
            MosgharlCreeperS => "Mosgharl Creeper S",
            SinisterGarb => "Sinister Garb",
            KushalaCocoon => "Kushala Cocoon",
            UtsushiBracesVS => "Utsushi Braces (V) S",
            SkullVisageS => "Skull Visage S",
            DamascusGreaves => "Damascus Greaves",
            BarrothVambraces => "Barroth Vambraces",
            KuluYaKuHelm => "Kulu-Ya-Ku Helm",
            LeatherVest => "Leather Vest",
            GossHaragBraces => "Goss Harag Braces",
            DamascusCoil => "Damascus Coil",
            HuntersMail => "Hunter's Mail",
            DeathStenchHeel => "Death Stench Heel",
            LagombiHelm => "Lagombi Helm",
            MediumsHairtie => "Medium's Hair-tie",
            VaikCoilS => "Vaik Coil S",
            JellyHatS => "Jelly Hat S",
            AlmudronGreavesS => "Almudron Greaves S",
            RemobraGlovesS => "Remobra Gloves S",
            NargacugaMail => "Nargacuga Mail",
            LeatherVestS => "Leather Vest S",
            RathianHelmS => "Rathian Helm S",
            GoldenObi => "Golden Obi",
            KamuraObi => "Kamura Obi",
            LudrothGreavesS => "Ludroth Greaves S",
            ChannelersHakamaS => "Channeler's Hakama S",
            ShelledSandals => "Shelled Sandals",
            NargacugaBracesS => "Nargacuga Braces S",
            NargacugaCoilS => "Nargacuga Coil S",
            DiablosCoilS => "Diablos Coil S",
            EdelCreeperS => "Edel Creeper S",
            LeatherHeadgearS => "Leather Headgear S",
            ChannelersHakama => "Channeler's Hakama",
            VolvidonMail => "Volvidon Mail",
            AelucanthVertex => "Aelucanth Vertex",
            ChannelersObiS => "Channeler's Obi S",
            MakluvaCoilS => "Makluva Coil S",
            LagombiVambracesS => "Lagombi Vambraces S",
            DamascusHelm => "Damascus Helm",
            WroggiGreaves => "Wroggi Greaves",
            GossHaragGreavesS => "Goss Harag Greaves S",
            KaiserCoil => "Kaiser Coil",
            KuluYaKuCoilS => "Kulu-Ya-Ku Coil S",
            LagombiCoil => "Lagombi Coil",
            RemobraGloves => "Remobra Gloves",
            BishatenHelmS => "Bishaten Helm S",
            UtsushiMaskHS => "Utsushi Mask (H) S",
            GossHaragMail => "Goss Harag Mail",
            BasariosHelm => "Basarios Helm",
            AlmudronVambraces => "Almudron Vambraces",
            RathianCoilS => "Rathian Coil S",
            BaggiHelmS => "Baggi Helm S",
            SkaldaElytra => "Skalda Elytra",
            BrigadeSuitS => "Brigade Suit S",
            KhezuVambraces => "Khezu Vambraces",
            DiablosMailS => "Diablos Mail S",
            AnjanathMailS => "Anjanath Mail S",
            RemobraFeetS => "Remobra Feet S",
            WroggiVambracesS => "Wroggi Vambraces S",
            KuluYaKuBracesS => "Kulu-Ya-Ku Braces S",
            ChannelersRobeS => "Channeler's Robe S",
            BishatenGreaves => "Bishaten Greaves",
            RemobraBeltS => "Remobra Belt S",
            SpioCruraS => "Spio Crura S",
            KuluYaKuGreaves => "Kulu-Ya-Ku Greaves",
            AelucanthCrura => "Aelucanth Crura",
            KhezuVambracesS => "Khezu Vambraces S",
            JyuratodusVambraces => "Jyuratodus Vambraces",
            LudrothCoilS => "Ludroth Coil S",
            MosgharlRibplate => "Mosgharl Ribplate",
            EdelRibplate => "Edel Ribplate",
            BasariosGreavesS => "Basarios Greaves S",
            AlloyHelm => "Alloy Helm",
            UroktorCoil => "Uroktor Coil",
            RemobraFeet => "Remobra Feet",
            IngotMailS => "Ingot Mail S",
            EdelRoots => "Edel Roots",
            DoberHelm => "Dober Helm",
            MediumsObi => "Medium's Obi",
            SStuddedGlovesS => "S. Studded Gloves S",
            IbushisPauldrons => "Ibushi's Pauldrons",
            LagombiVambraces => "Lagombi Vambraces",
            UtsushiMaskH => "Utsushi Mask (H)",
            IngotVambracesS => "Ingot Vambraces S",
            LudrothMail => "Ludroth Mail",
            ChromeMetalBoots => "Chrome Metal Boots",
            TigrexCoil => "Tigrex Coil",
            MizuhaGuards => "Mizuha Guards",
            RhenoplosHelmS => "Rhenoplos Helm S",
            KhezuHelmS => "Khezu Helm S",
            MizutsuneBracesS => "Mizutsune Braces S",
            HuntersCoil => "Hunter's Coil",
            MakluvaCoverS => "Makluva Cover S",
            MelahoaFolia => "Melahoa Folia",
            JellyCoil => "Jelly Coil",
            BullfangoMaskS => "Bullfango Mask S",
            BariothGreavesS => "Barioth Greaves S",
            AlmudronMailS => "Almudron Mail S",
            AlloyGreavesS => "Alloy Greaves S",
            SomnacanthHelm => "Somnacanth Helm",
            IzuchiCoilS => "Izuchi Coil S",
            AlmudronCoil => "Almudron Coil",
            TetranadonMailS => "Tetranadon Mail S",
            SkaldaCrura => "Skalda Crura",
            SpioElytraS => "Spio Elytra S",
            NargacugaBraces => "Nargacuga Braces",
            BaggiVambracesS => "Baggi Vambraces S",
            BishatenGreavesS => "Bishaten Greaves S",
            UtsushiGreavesV => "Utsushi Greaves (V)",
            UtsushiGreavesVS => "Utsushi Greaves (V) S",
            TigrexGreaves => "Tigrex Greaves",
            GossHaragBracesS => "Goss Harag Braces S",
            WroggiCoil => "Wroggi Coil",
            MosgharlCreeper => "Mosgharl Creeper",
            RemobraBelt => "Remobra Belt",
            JellyBootsS => "Jelly Boots S",
            LagombiMail => "Lagombi Mail",
            JaggiGauntletsS => "Jaggi Gauntlets S",
            JaggiMask => "Jaggi Mask",
            RemobraSuit => "Remobra Suit",
            EdelFoliaS => "Edel Folia S",
            BoneHelmS => "Bone Helm S",
            ArzurosHelm => "Arzuros Helm",
            DiablosMail => "Diablos Mail",
            BarrothMailS => "Barroth Mail S",
            RathalosGreavesS => "Rathalos Greaves S",
            AelucanthVertexS => "Aelucanth Vertex S",
            MakluvaCoil => "Makluva Coil",
            TigrexHelmS => "Tigrex Helm S",
            ShelledSandalsS => "Shelled Sandals S",
            TobiKadachiBraces => "Tobi-Kadachi Braces",
            MizutsuneGreavesS => "Mizutsune Greaves S",
            IbushisHelm => "Ibushi's Helm",
            DrothMailS => "Droth Mail S",
            SkaldaThorax => "Skalda Thorax",
            MosgharlVizorS => "Mosgharl Vizor S",
            RhopessaThoraxS => "Rhopessa Thorax S",
            UtsushiMaskVS => "Utsushi Mask (V) S",
            ZinogreHelmS => "Zinogre Helm S",
            BariothCoil => "Barioth Coil",
            TheaterWig => "Theater Wig",
            MizutsuneMailS => "Mizutsune Mail S",
            BariothVambracesS => "Barioth Vambraces S",
            SinisterSealBraces => "Sinister Seal Braces",
            BariothMail => "Barioth Mail",
            RathianBraces => "Rathian Braces",
            MakluvaSleeves => "Makluva Sleeves",
            BoneHelm => "Bone Helm",
            ArzurosMail => "Arzuros Mail",
            UtsushiBracesHS => "Utsushi Braces (H) S",
            SomnacanthMailS => "Somnacanth Mail S",
            AlloyCoilS => "Alloy Coil S",
            HuntersGreaves => "Hunter's Greaves",
            RathalosBraces => "Rathalos Braces",
            KamuraiGarb => "Kamurai Garb",
            MelahoaHat => "Melahoa Hat",
            MosgharlRootsS => "Mosgharl Roots S",
            SomnacanthBracesS => "Somnacanth Braces S",
            AlloyGreaves => "Alloy Greaves",
            ZinogreMailS => "Zinogre Mail S",
            GoldenHakama => "Golden Hakama",
            RathalosGreaves => "Rathalos Greaves",
            NarwaBreastplate => "Narwa Breastplate",
            ValstraxBraces => "Valstrax Braces",
            LeatherGlovesS => "Leather Gloves S",
            UroktorTorsoS => "Uroktor Torso S",
            WroggiHelmS => "Wroggi Helm S",
            GoldenKote => "Golden Kote",
            RathalosBracesS => "Rathalos Braces S",
            KhezuMailS => "Khezu Mail S",
            JaggiMaskS => "Jaggi Mask S",
            MizuhaSash => "Mizuha Sash",
            BarrothHelm => "Barroth Helm",
            BaggiGreaves => "Baggi Greaves",
            AknosomBraces => "Aknosom Braces",
            VolvidonCoilS => "Volvidon Coil S",
            BasariosMailS => "Basarios Mail S",
            PukeiPukeiBracesS => "Pukei-Pukei Braces S",
            UtsushiMaskV => "Utsushi Mask (V)",
            JellyVest => "Jelly Vest",
            AknosomCoilS => "Aknosom Coil S",
            AelucanthBrachiaS => "Aelucanth Brachia S",
            TetranadonGreavesS => "Tetranadon Greaves S",
            BnahabraSuitS => "Bnahabra Suit S",
            IzuchiBraces => "Izuchi Braces",
            AlmudronGreaves => "Almudron Greaves",
            DiablosGreaves => "Diablos Greaves",
            HuntersVambraces => "Hunter's Vambraces",
            BnahabraGlovesS => "Bnahabra Gloves S",
            DeathStenchMuscle => "Death Stench Muscle",
            AlloyCoil => "Alloy Coil",
            TetranadonGreaves => "Tetranadon Greaves",
            KamuraiBraces => "Kamurai Braces",
            TetranadonMail => "Tetranadon Mail",
            RathalosCoil => "Rathalos Coil",
            DeathStenchHeelS => "Death Stench Heel S",
            KushalaGlare => "Kushala Glare",
            ArzurosCoilS => "Arzuros Coil S",
            ShellStuddedVestS => "Shell-Studded Vest S",
            ValstraxMail => "Valstrax Mail",
            ChaosPlate => "Chaos Plate",
            ChainmailPantsS => "Chainmail Pants S",
            AnjanathGreavesS => "Anjanath Greaves S",
            BullfangoMask => "Bullfango Mask",
            SpioCrura => "Spio Crura",
            RathianCoil => "Rathian Coil",
            MediumsHakama => "Medium's Hakama",
            VolvidonGreaves => "Volvidon Greaves",
            BrigadeBoots => "Brigade Boots",
            ArzurosVambraces => "Arzuros Vambraces",
            VolvidonHelmS => "Volvidon Helm S",
            DStenchMuscleS => "D. Stench Muscle S",
            ChainmailGlovesS => "Chainmail Gloves S",
            IngotMail => "Ingot Mail",
            SpioVertexS => "Spio Vertex S",
            BnahabraBootsS => "Bnahabra Boots S",
            FootofNarwa => "Foot of Narwa",
            MizuhaCap => "Mizuha Cap",
            ChannelersHairtie => "Channeler's Hair-tie",
            UroktorCoilS => "Uroktor Coil S",
            SinisterTassets => "Sinister Tassets",
            JellyHat => "Jelly Hat",
            AlmudronHelm => "Almudron Helm",
            KaiserVambraces => "Kaiser Vambraces",
            FeatherofMastery => "Feather of Mastery",
            IngotCoil => "Ingot Coil",
            JyuratodusHelm => "Jyuratodus Helm",
            RhopessaBrachiaS => "Rhopessa Brachia S",
            MelahoaRoots => "Melahoa Roots",
            MosgharlVizor => "Mosgharl Vizor",
            UtsushiChestHS => "Utsushi Chest (H) S",
            SomnacanthMail => "Somnacanth Mail",
            ArzurosMailS => "Arzuros Mail S",
            LudrothMailS => "Ludroth Mail S",
            AnjanathMail => "Anjanath Mail",
            RathalosMailS => "Rathalos Mail S",
            LudrothCoil => "Ludroth Coil",
            SpioBrachiaS => "Spio Brachia S",
            IngotHelm => "Ingot Helm",
            AnjanathCoilS => "Anjanath Coil S",
            IzuchiBracesS => "Izuchi Braces S",
            UtsushiTassetsVS => "Utsushi Tassets (V) S",
            DeathStenchBrain => "Death Stench Brain",
            VaikHelm => "Vaik Helm",
            BasariosCoil => "Basarios Coil",
            TigrexBraces => "Tigrex Braces",
            DiablosHelm => "Diablos Helm",
            BarrothVambracesS => "Barroth Vambraces S",
            MakluvaSleevesS => "Makluva Sleeves S",
            GossHaragMailS => "Goss Harag Mail S",
            ZinogreCoilS => "Zinogre Coil S",
            MizutsuneHelm => "Mizutsune Helm",
            ChainmailBelt => "Chainmail Belt",
            BasariosMail => "Basarios Mail",
            RathalosHelm => "Rathalos Helm",
            MelahoaHatS => "Melahoa Hat S",
            DiablosGreavesS => "Diablos Greaves S",
            AnjanathVambracesS => "Anjanath Vambraces S",
            SlagtothHood => "Slagtoth Hood",
            TigrexHelm => "Tigrex Helm",
            JellyCoilS => "Jelly Coil S",
            GossHaragGreaves => "Goss Harag Greaves",
            RathalosMail => "Rathalos Mail",
            IngotGreaves => "Ingot Greaves",
            SkaldaElytraS => "Skalda Elytra S",
            IzuchiHelmS => "Izuchi Helm S",
            ChannelersRobe => "Channeler's Robe",
            MelahoaJacket => "Melahoa Jacket",
            MosgharlRoots => "Mosgharl Roots",
            SomnacanthGreavesS => "Somnacanth Greaves S",
            KaiserCrown => "Kaiser Crown",
            AlmudronCoilS => "Almudron Coil S",
            BazelgeuseGreaves => "Bazelgeuse Greaves",
            LudrothBraces => "Ludroth Braces",
            BaggiMailS => "Baggi Mail S",
            SinisterGreavesS => "Sinister Greaves S",
            BnahabraGloves => "Bnahabra Gloves",
            MosgharlFolia => "Mosgharl Folia",
            BarrothGreaves => "Barroth Greaves",
            PukeiPukeiBraces => "Pukei-Pukei Braces",
            ZinogreHelm => "Zinogre Helm",
            BoneMailS => "Bone Mail S",
            EdelFolia => "Edel Folia",
            IzuchiHelm => "Izuchi Helm",
            UtsushiChestV => "Utsushi Chest (V)",
            KamuraBracesS => "Kamura Braces S",
            RhopessaThorax => "Rhopessa Thorax",
            KhezuCoilS => "Khezu Coil S",
            SpioThorax => "Spio Thorax",
            SwallowShirt => "Swallow Shirt",
            KadachiGreaves => "Kadachi Greaves",
            SkaldaBrachiaS => "Skalda Brachia S",
            RhopessaCruraS => "Rhopessa Crura S",
            BrigadeCoilS => "Brigade Coil S",
            KamuraGarbS => "Kamura Garb S",
            VolvidonHelm => "Volvidon Helm",
            MelahoaRootsS => "Melahoa Roots S",
            RathalosCoilS => "Rathalos Coil S",
            LagombiGreavesS => "Lagombi Greaves S",
            ShellStuddedVest => "Shell-Studded Vest",
            TetranadonHelmS => "Tetranadon Helm S",
            DoberVambraces => "Dober Vambraces",
            LeatherBelt => "Leather Belt",
            SomnacanthHelmS => "Somnacanth Helm S",
            RathianMailS => "Rathian Mail S",
            NargacugaHelmS => "Nargacuga Helm S",
            JaggiShinguards => "Jaggi Shinguards",
            AnjanathHelm => "Anjanath Helm",
            BariothMailS => "Barioth Mail S",
            DoberMail => "Dober Mail",
            GossHaragCoil => "Goss Harag Coil",
            KhezuHelm => "Khezu Helm",
            LeatherBeltS => "Leather Belt S",
            DiablosVambracesS => "Diablos Vambraces S",
            SinisterGauntlets => "Sinister Gauntlets",
            NargacugaHelm => "Nargacuga Helm",
            KushalaCrus => "Kushala Crus",
            RhenoplosCoil => "Rhenoplos Coil",
            SkaldaVertexS => "Skalda Vertex S",
            BnahabraHat => "Bnahabra Hat",
            FlameSeal => "Flame Seal",
            BrigadeCoil => "Brigade Coil",
            BishatenCoil => "Bishaten Coil",
            ZinogreMail => "Zinogre Mail",
            VaikCoil => "Vaik Coil",
            RhenoplosHelm => "Rhenoplos Helm",
            AknosomCoil => "Aknosom Coil",
            SStuddedHat => "S. Studded Hat",
            LeatherPants => "Leather Pants",
            ChainmailVestS => "Chainmail Vest S",
            ChromeMetalCoil => "Chrome Metal Coil",
            DeathStenchBowels => "Death Stench Bowels",
            ZinogreBraces => "Zinogre Braces",
            KadachiGreavesS => "Kadachi Greaves S",
            BishatenMail => "Bishaten Mail",
            ChainmailHeadgear => "Chainmail Headgear",
            JellyGloves => "Jelly Gloves",
            MizutsuneCoil => "Mizutsune Coil",
            RhenoplosGreavesS => "Rhenoplos Greaves S",
            TobiKadachiCoilS => "Tobi-Kadachi Coil S",
            RaknaHelm => "Rakna Helm",
            TigrexBracesS => "Tigrex Braces S",
            MizuhaGaiters => "Mizuha Gaiters",
            SinisterSealGreaves => "Sinister Seal Greaves",
            KamuraObiS => "Kamura Obi S",
            PukeiPukeiCoil => "Pukei-Pukei Coil",
            SinisterHelmS => "Sinister Helm S",
            AelucanthThorax => "Aelucanth Thorax",
            VolvidonCoil => "Volvidon Coil",
            RathianGreaves => "Rathian Greaves",
            IbushisFauld => "Ibushi's Fauld",
            ChainmailHeadgearS => "Chainmail Headgear S",
            KadachiHelm => "Kadachi Helm",
            SpioBrachia => "Spio Brachia",
            NargacugaGreaves => "Nargacuga Greaves",
            NarwasFauld => "Narwa's Fauld",
            SomnacanthCoilS => "Somnacanth Coil S",
            SpioElytra => "Spio Elytra",
            MizutsuneGreaves => "Mizutsune Greaves",
            AnjanathHelmS => "Anjanath Helm S",
            TetranadonHelm => "Tetranadon Helm",
            SinisterGreaves => "Sinister Greaves",
            MelahoaFoliaS => "Melahoa Folia S",
            MizutsuneBraces => "Mizutsune Braces",
            MizutsuneHelmS => "Mizutsune Helm S",
            SinisterHelm => "Sinister Helm",
            BariothHelmS => "Barioth Helm S",
            LeatherHeadgear => "Leather Headgear",
            KamuraiLeggings => "Kamurai Leggings",
            BarrothCoilS => "Barroth Coil S",
            KaiserGreaves => "Kaiser Greaves",
            BrigadeLobosS => "Brigade Lobos S",
            DrothGreavesS => "Droth Greaves S",
            UtsushiChestVS => "Utsushi Chest (V) S",
            LagombiGreaves => "Lagombi Greaves",
            PukeiPukeiMail => "Pukei-Pukei Mail",
            BoneVambraces => "Bone Vambraces",
            KamuraHeadScarfS => "Kamura Head Scarf S",
            VolvidonVambraces => "Volvidon Vambraces",
            ChaosArchbun => "Chaos Archbun",
            ArzurosVambracesS => "Arzuros Vambraces S",
            SkaldaThoraxS => "Skalda Thorax S",
            RaknaMail => "Rakna Mail",
            IbushisBreastplate => "Ibushi's Breastplate",
            BnahabraCoilS => "Bnahabra Coil S",
            DStenchBowelsS => "D. Stench Bowels S",
            KadachiHelmS => "Kadachi Helm S",
            SinisterGauntletsS => "Sinister Gauntlets S",
            ArzurosCoil => "Arzuros Coil",
            SlagtothCloak => "Slagtoth Cloak",
            DoberGreaves => "Dober Greaves",
            AknosomMail => "Aknosom Mail",
            LagombiMailS => "Lagombi Mail S",
            NargacugaGreavesS => "Nargacuga Greaves S",
            ZinogreBracesS => "Zinogre Braces S",
            BariothVambraces => "Barioth Vambraces",
            SpioThoraxS => "Spio Thorax S",
            MediumsHairtieS => "Medium's Hair-tie S",
            AlmudronVambracesS => "Almudron Vambraces S",
            AnjanathVambraces => "Anjanath Vambraces",
            EdelVizor => "Edel Vizor",
            KaiserMail => "Kaiser Mail",
            GoldenHeaddress => "Golden Headdress",
            MakluvaHood => "Makluva Hood",
            IzuchiGreaves => "Izuchi Greaves",
            KuluYaKuMail => "Kulu-Ya-Ku Mail",
            TetranadonCoil => "Tetranadon Coil",
            RhenoplosMail => "Rhenoplos Mail",
            BazelgeuseCoil => "Bazelgeuse Coil",
            VolvidonMailS => "Volvidon Mail S",
            PukeiPukeiHelm => "Pukei-Pukei Helm",
            AlloyMail => "Alloy Mail",
            ChannelersHairtieS => "Channeler's Hair-tie S",
            SkaldaBrachia => "Skalda Brachia",
            RemobraSuitS => "Remobra Suit S",
            IngotCoilS => "Ingot Coil S",
            JyuratodusCoil => "Jyuratodus Coil",
            LudrothHelm => "Ludroth Helm",
            AelucanthBrachia => "Aelucanth Brachia",
            LudrothHelmS => "Ludroth Helm S",
            KuluYaKuHelmS => "Kulu-Ya-Ku Helm S",
            AlmudronMail => "Almudron Mail",
            UtsushiGreavesHS => "Utsushi Greaves (H) S",
            BnahabraSuit => "Bnahabra Suit",
            JyuratodusMail => "Jyuratodus Mail",
            BazelgeuseMail => "Bazelgeuse Mail",
            DiablosCoil => "Diablos Coil",
            MakluvaPantsS => "Makluva Pants S",
            UtsushiTassetsHS => "Utsushi Tassets (H) S",
            DrothCoilS => "Droth Coil S",
            BishatenCoilS => "Bishaten Coil S",
            PukeiPukeiCoilS => "Pukei-Pukei Coil S",
            VaikMail => "Vaik Mail",
            ChainmailBeltS => "Chainmail Belt S",
            BarrothMail => "Barroth Mail",
            SinisterGarbS => "Sinister Garb S",
            MizutsuneCoilS => "Mizutsune Coil S",
            VaikHelmS => "Vaik Helm S",
            BrigadeVambracesS => "Brigade Vambraces S",
            MizuhaSleeves => "Mizuha Sleeves",
            BishatenVambracesS => "Bishaten Vambraces S",
            DamascusVambraces => "Damascus Vambraces",
            PukeiPukeiMailS => "Pukei-Pukei Mail S",
            SomnacanthCoil => "Somnacanth Coil",
            AknosomGreavesS => "Aknosom Greaves S",
            UtsushiTassetsH => "Utsushi Tassets (H)",
            BnahabraBoots => "Bnahabra Boots",
            WroggiMailS => "Wroggi Mail S",
            KamuraBraces => "Kamura Braces",
            ShellStuddedGloves => "Shell-Studded Gloves",
            BoneCoilS => "Bone Coil S",
            BaggiCoilS => "Baggi Coil S",
            SinisterTassetsS => "Sinister Tassets S",
            EdelVizorS => "Edel Vizor S",
            NarwasPauldrons => "Narwa's Pauldrons",
            UroktorTorso => "Uroktor Torso",
            UtsushiGreavesH => "Utsushi Greaves (H)",
            UtsushiBracesV => "Utsushi Braces (V)",
            RhopessaElytra => "Rhopessa Elytra",
            HuntersGreavesS => "Hunter's Greaves S",
            ValstraxCoil => "Valstrax Coil",
            GargwaMaskS => "Gargwa Mask S",
            JaggiGauntlets => "Jaggi Gauntlets",
            DoberCoil => "Dober Coil",
            GossHaragHelm => "Goss Harag Helm",
            BaggiGreavesS => "Baggi Greaves S",
            SkullVisage => "Skull Visage",
            RaknaGreaves => "Rakna Greaves",
            TigrexMailS => "Tigrex Mail S",
            CunningSpecs => "Cunning Specs",
            TobiKadachiMail => "Tobi-Kadachi Mail",
            GossHaragCoilS => "Goss Harag Coil S",
            BishatenMailS => "Bishaten Mail S",
            FootofIbushi => "Foot of Ibushi",
            BasariosVambracesS => "Basarios Vambraces S",
            SpioVertex => "Spio Vertex",
            BoneCoil => "Bone Coil",
            TigrexMail => "Tigrex Mail",
            DrothGreaves => "Droth Greaves",
            PukeiGreavesS => "Pukei Greaves S",
            RhenoplosMailS => "Rhenoplos Mail S",
            KuluYaKuMailS => "Kulu-Ya-Ku Mail S",
            BazelgeuseHelm => "Bazelgeuse Helm",
            KamuraLeggings => "Kamura Leggings",
            ArzurosGreavesS => "Arzuros Greaves S",
            IngotGreavesS => "Ingot Greaves S",
            KuluYaKuGreavesS => "Kulu-Ya-Ku Greaves S",
            BaggiHelm => "Baggi Helm",
            KhezuGreavesS => "Khezu Greaves S",
            TigrexGreavesS => "Tigrex Greaves S",
            MelahoaBranch => "Melahoa Branch",
            RaknaCoil => "Rakna Coil",
            AknosomGreaves => "Aknosom Greaves",
            ZinogreGreavesS => "Zinogre Greaves S",
            SinisterSealGarb => "Sinister Seal Garb",
            BariothGreaves => "Barioth Greaves",
            LeatherPantsS => "Leather Pants S",
            RemobraHeadgearS => "Remobra Headgear S",
            ZinogreGreaves => "Zinogre Greaves",
            RhopessaBrachia => "Rhopessa Brachia",
            SStuddedSash => "S. Studded Sash",
            MediumsPrayerS => "Medium's Prayer S",
            KamuraLeggingsS => "Kamura Leggings S",
            TobiKadachiMailS => "Tobi-Kadachi Mail S",
            AknosomHelmS => "Aknosom Helm S",
            MelahoaBranchS => "Melahoa Branch S",
            SStuddedHatS => "S. Studded Hat S",
            AlloyVambraces => "Alloy Vambraces",
            BnahabraHatS => "Bnahabra Hat S",
            RathianBracesS => "Rathian Braces S",
            ValstraxHelm => "Valstrax Helm",
            IngotVambraces => "Ingot Vambraces",
            AlloyVambracesS => "Alloy Vambraces S",
            RhopessaCrura => "Rhopessa Crura",
            SinisterSealObi => "Sinister Seal Obi",
            GargwaMask => "Gargwa Mask",
            KamuraiHat => "Kamurai Hat",
            BaggiVambraces => "Baggi Vambraces",
            KuluYaKuBraces => "Kulu-Ya-Ku Braces",
            BoneGreaves => "Bone Greaves",
            MakluvaCover => "Makluva Cover",
            TetranadonBracesS => "Tetranadon Braces S",
            VaikBracesS => "Vaik Braces S",
            NargacugaMailS => "Nargacuga Mail S",
            KushalaCista => "Kushala Cista",
            RathianMail => "Rathian Mail",
            RhenoplosBraces => "Rhenoplos Braces",
            ChaosArchplate => "Chaos Archplate",
            IzuchiCoil => "Izuchi Coil",
            AelucanthThoraxS => "Aelucanth Thorax S",
            FoxMask => "Fox Mask",
            KamuraHeadScarf => "Kamura Head Scarf",
            SwallowGloves => "Swallow Gloves",
            LudrothGreaves => "Ludroth Greaves",
            HuntersVambracesS => "Hunter's Vambraces S",
            SlagtothCloakS => "Slagtoth Cloak S",
            VolvidonGreavesS => "Volvidon Greaves S",
            DiablosHelmS => "Diablos Helm S",
            MakluvaPants => "Makluva Pants",
            MightyBowFeather => "Mighty Bow Feather",
            BoneMail => "Bone Mail",
            ArzurosHelmS => "Arzuros Helm S",
            EdelCreeper => "Edel Creeper",
            ArzurosGreaves => "Arzuros Greaves",
            NargacugaCoil => "Nargacuga Coil",
            SomnacanthGreaves => "Somnacanth Greaves",
            DrothMail => "Droth Mail",
            ChainmailPants => "Chainmail Pants",
            DeathStenchBrainS => "Death Stench Brain S",
            NarwasHelm => "Narwa's Helm",
            PukeiPukeiHelmS => "Pukei-Pukei Helm S",
            WroggiMail => "Wroggi Mail",
            GossHaragHelmS => "Goss Harag Helm S",
            RemobraHeadgear => "Remobra Headgear",
            KhezuGreaves => "Khezu Greaves",
            BarrothCoil => "Barroth Coil",
            ChannelersHopeS => "Channeler's Hope S",
            ChainmailVest => "Chainmail Vest",
            AelucanthCruraS => "Aelucanth Crura S",
            JellyVestS => "Jelly Vest S",
            DamascusMail => "Damascus Mail",
            BishatenVambraces => "Bishaten Vambraces",
            WroggiHelm => "Wroggi Helm",
            SkaldaVertex => "Skalda Vertex",
            HuntersHelm => "Hunter's Helm",
            AknosomHelm => "Aknosom Helm",
            DrothCoil => "Droth Coil",
            BoneVambracesS => "Bone Vambraces S",
            KamuraGarb => "Kamura Garb",
            RhopessaVertexS => "Rhopessa Vertex S",
            AnjanathGreaves => "Anjanath Greaves",
            IzuchiMail => "Izuchi Mail",
            ChannelersObi => "Channeler's Obi",
            SwallowBoots => "Swallow Boots",
            PukeiGreaves => "Pukei Greaves",
            ChainmailGloves => "Chainmail Gloves",
            BaggiCoil => "Baggi Coil",
            BishatenHelm => "Bishaten Helm",
            HuntersCoilS => "Hunter's Coil S",
            BazelgeuseBraces => "Bazelgeuse Braces",
            VaikMailS => "Vaik Mail S",
            DeathStenchGrip => "Death Stench Grip",
            AlloyHelmS => "Alloy Helm S",
            TetranadonBraces => "Tetranadon Braces",
            KamuraiObi => "Kamurai Obi",
            RaknaArmguards => "Rakna Armguards",
            TetranadonCoilS => "Tetranadon Coil S",
            AknosomBracesS => "Aknosom Braces S",
            ValstraxGreaves => "Valstrax Greaves",
            UtsushiBracesH => "Utsushi Braces (H)",
            BasariosGreaves => "Basarios Greaves",
            SlagtothHoodS => "Slagtoth Hood S",
            BoneGreavesS => "Bone Greaves S",
            BrigadeBootsS => "Brigade Boots S",
            BarrothHelmS => "Barroth Helm S",
            KhezuCoil => "Khezu Coil",
            MediumsPrayer => "Medium's Prayer",
            VolvidonVambracesS => "Volvidon Vambraces S",
            DeathStenchGripS => "Death Stench Grip S",
            SomnacanthBraces => "Somnacanth Braces",
            ZinogreCoil => "Zinogre Coil",
            AelucanthElytra => "Aelucanth Elytra",
            WroggiGreavesS => "Wroggi Greaves S",
            RhenoplosBracesS => "Rhenoplos Braces S",
            BrigadeLobos => "Brigade Lobos",
            VaikGreavesS => "Vaik Greaves S",
            BariothHelm => "Barioth Helm",
            JellyGlovesS => "Jelly Gloves S",
            WroggiVambraces => "Wroggi Vambraces",
            LagombiCoilS => "Lagombi Coil S",
            MediumsObiS => "Medium's Obi S",
            MediumsRobeS => "Medium's Robe S",
            AknosomMailS => "Aknosom Mail S",
            AlmudronHelmS => "Almudron Helm S",
            AelucanthElytraS => "Aelucanth Elytra S",
            SinisterSealMask => "Sinister Seal Mask",
            RhenoplosGreaves => "Rhenoplos Greaves",
            RathianGreavesS => "Rathian Greaves S",
            BariothCoilS => "Barioth Coil S",
            UtsushiChestH => "Utsushi Chest (H)",
            IzuchiMailS => "Izuchi Mail S",
            LeatherGloves => "Leather Gloves",
            MakluvaHoodS => "Makluva Hood S",
            BrigadeVambraces => "Brigade Vambraces",
            KhezuMail => "Khezu Mail",
            TigrexCoilS => "Tigrex Coil S",
            BaggiMail => "Baggi Mail",
            VaikGreaves => "Vaik Greaves",
            BrigadeSuit => "Brigade Suit",
            TobiKadachiCoil => "Tobi-Kadachi Coil",
            MediumsRobe => "Medium's Robe",
            DiablosVambraces => "Diablos Vambraces",
            BasariosVambraces => "Basarios Vambraces",
            LudrothBracesS => "Ludroth Braces S",
            UtsushiTassetsV => "Utsushi Tassets (V)",
            AlloyMailS => "Alloy Mail S",
            Chaoshroom => "Chaoshroom",
            WroggiCoilS => "Wroggi Coil S",
            HuntersHelmS => "Hunter's Helm S",
            HuntersMailS => "Hunter's Mail S",
            MizutsuneMail => "Mizutsune Mail",
            IngotHelmS => "Ingot Helm S",
            BasariosHelmS => "Basarios Helm S",
            JellyBoots => "Jelly Boots",
            JyuratodusGreaves => "Jyuratodus Greaves",
            GoldenHaori => "Golden Haori",
            JaggiShinguardsS => "Jaggi Shinguards S",
            KuluYaKuCoil => "Kulu-Ya-Ku Coil",
            BarrothGreavesS => "Barroth Greaves S",
            LagombiHelmS => "Lagombi Helm S",
            SkaldaCruraS => "Skalda Crura S",
            BasariosCoilS => "Basarios Coil S",
        }
    }
}
