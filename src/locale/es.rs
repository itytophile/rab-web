pub trait Spanish {
    fn to_spanish(&self) -> &'static str;
}

impl Spanish for crate::DisplaySkill {
    fn to_spanish(&self) -> &'static str {
        use rab_core::armor_and_skills::Skill::*;

        match self.0 {
            AttackBoost => "Ataque",
            Agitator => "Instigador",
            PeakPerformance => "Plena forma",
            Resentment => "Resentimiento",
            Resuscitate => "Resistencia",
            CriticalEye => "Ojo crítico",
            CriticalBoost => "Potenciador crítico",
            WeaknessExploit => "Punto débil",
            LatentPower => "Poder latente",
            MaximumMight => "Afinidad",
            CriticalElement => "Elemento crítico",
            MastersTouch => "Toque del maestro",
            FireAttack => "Ataque de fuego",
            WaterAttack => "Ataque de agua",
            IceAttack => "Ataque de hielo",
            ThunderAttack => "Ataque de rayo",
            DragonAttack => "Ataque de draco",
            PoisonAttack => "Ataque venenoso",
            ParalysisAttack => "Ataque paralizante",
            SleepAttack => "Ataque de sueño",
            BlastAttack => "Ataque nitro",
            Handicraft => "Artesanía",
            RazorSharp => "Filo de cuchilla",
            SpareShot => "Tiro extra",
            ProtectivePolish => "Acabado protector",
            MindsEye => "Ojo mental",
            Ballistics => "Balística",
            Bludgeoner => "Impacto mejorado",
            BowChargePlus => "Arquero maestro",
            Focus => "Tiempo de carga",
            PowerProlonger => "Carga prolongada",
            MarathonRunner => "Esprint",
            Constitution => "Constitución",
            StaminaSurge => "Adrenalina",
            Guard => "Guardia mejorada",
            GuardUp => "Escudo",
            OffensiveGuard => "Bloqueo agresivo",
            CriticalDraw => "Desenfundar",
            PunishingDraw => "Pena inmediata",
            QuickSheath => "Desenfunde veloz",
            Slugger => "Aturdir",
            StaminaThief => "Agotador",
            AffinitySliding => "Afinidad deslizante",
            HornMaestro => "Músico",
            Artillery => "Artillería",
            LoadShells => "Cargar arma",
            SpecialAmmoBoost => "Tiro especial",
            NormalRapidUp => "Normal/salva+",
            PierceUp => "Perforante+",
            SpreadUp => "Abanico+",
            AmmoUp => "Munición+",
            ReloadSpeed => "Recarga",
            RecoilDown => "Cadencia",
            Steadiness => "Estabilidad",
            RapidFireUp => "Disparo múltiple",
            DefenseBoost => "Aumento defensivo",
            DivineBlessing => "Bendición divina",
            RecoveryUp => "Recuperación",
            RecoverySpeed => "Recuperación rápida",
            SpeedEating => "Consumir",
            Earplugs => "Tapones",
            Windproof => "Antiviento",
            TremorResistance => "Antitemblores",
            BubblyDance => "Danza burbujeante",
            EvadeWindow => "Intervalo evasivo",
            EvadeExtender => "Distancia evasiva",
            FireResistance => "Antifuego",
            WaterResistance => "Antiagua",
            IceResistance => "Antihielo",
            ThunderResistance => "Antirrayo",
            DragonResistance => "Antidraco",
            BlightResistance => "Antiplagas",
            PoisonResistance => "Antiveneno",
            ParalysisResistance => "Antiparálisis",
            SleepResistance => "Antisueño",
            StunResistance => "Antiaturdimiento",
            MuckResistance => "Antifango",
            BlastResistance => "Antinitro",
            Botanist => "Botánico",
            Geologist => "Geólogo",
            Partbreaker => "Rompepartes",
            CaptureMaster => "Maestro capturas",
            CarvingMaster => "Despiece extra",
            GoodLuck => "Afortunado",
            SpeedSharpening => "Afilado rápido",
            Bombardier => "Bombardero",
            Mushroomancer => "Experto en setas",
            ItemProlonger => "Duración de objetos",
            WideRange => "Efecto área",
            FreeMeal => "Comida gratis",
            Heroics => "Heroicidad",
            Fortify => "Fortalecer",
            FlinchFree => "Mejora de balance",
            JumpMaster => "Experto en salto",
            CarvingPro => "Experto cortador",
            HungerResistance => "Gasto resistencia",
            LeapofFaith => "Mejora de salto",
            Diversion => "Distracción",
            MasterMounter => "As de la monta",
            ChameleosBlessing => "Bendición de Chameleos",
            KushalaBlessing => "Bendición de Kushala",
            TeostraBlessing => "Bendición de Teostra",
            WirebugWhisperer => "Domacordópteros",
            WallRunner => "Correparedes",
            Counterstrike => "Contraataque",
            RapidMorph => "Mutación rápida",
            HellfireCloak => "Manto infernal",
            WindAlignment => "Viento unificado",
            ThunderAlignment => "Rayo unificado",
            Dragonheart => "Corazón de dragón",
            Stormsoul => "Viento y trueno",
        }
    }
}

impl Spanish for crate::armors::Armor {
    fn to_spanish(&self) -> &'static str {
        use crate::armors::Armor::*;

        match self {
            LudrothHelmS => "Casco Ludroth S",
            SomnacanthGreavesS => "Grebas Somnacanth S",
            RathianGreavesS => "Grebas Rathian S",
            DoberHelm => "Yelmo de Dober",
            WroggiMail => "Cota Wroggi",
            KuluYaKuBraces => "Brazales Kulu-Ya-Ku",
            RhopessaThorax => "Tórax Rhopessa",
            ValstraxBraces => "Brazales Valstrax",
            KamuraLeggingsS => "Leotardos Kamura S",
            VaikMail => "Cota Vaik",
            AlloyVambracesS => "Brazales de aleación S",
            MizutsuneCoilS => "Faja Mizutsune S",
            BaggiHelm => "Casco Baggi",
            MakluvaCover => "Poncho Makluva",
            BnahabraBootsS => "Botas Bnahabra S",
            AlloyCoilS => "Faja de aleación S",
            GoldenKote => "Kote dorado",
            IbushisFauld => "Muslera Ibushi",
            MakluvaSleevesS => "Mangas Makluva S",
            SkullVisageS => "Rostro calavérico S",
            PukeiPukeiCoilS => "Faja Pukei-Pukei S",
            SinisterHelm => "Casco siniestro",
            TetranadonMail => "Cota Tetranadon",
            MelahoaBranch => "Rama Melahoa",
            VolvidonMailS => "Cota Volvidon S",
            BariothMailS => "Cota Barioth S",
            NargacugaCoil => "Faja Nargacuga",
            JellyCoilS => "Cinto gelatina S",
            TigrexGreavesS => "Grebas Tigrex S",
            IbushisHelm => "Casco Ibushi",
            KamuraGarb => "Vestimenta Kamura",
            MosgharlRoots => "Raíces Mosgharl",
            DrothCoil => "Faja Droth",
            SkaldaBrachia => "Ristre Skalda",
            WroggiGreavesS => "Grebas Wroggi S",
            DoberVambraces => "Brazales de Dober",
            MelahoaHatS => "Sombrero Melahoa S",
            BarrothCoil => "Faja Barroth",
            BariothMail => "Cota Barioth",
            UtsushiBracesVS => "Brazales Utsushi (V) S",
            GossHaragCoilS => "Faja Goss Harag S",
            AlmudronMailS => "Cota Almudron S",
            JellyBoots => "Botas gelatina",
            UtsushiTassetsHS => "Muslera Utsushi (O) S",
            BrigadeLobosS => "Sombrero de brigada S",
            KuluYaKuGreavesS => "Grebas Kulu-Ya-Ku S",
            ArzurosHelm => "Casco Arzuros",
            GossHaragHelm => "Casco Goss Harag",
            IngotHelm => "Yelmo de lingotes",
            ValstraxCoil => "Faja Valstrax",
            NargacugaGreavesS => "Grebas Nargacuga S",
            SpioElytra => "Élitro Spio",
            DrothMailS => "Cota Droth S",
            SomnacanthMailS => "Cota Somnacanth S",
            TobiKadachiCoilS => "Faja Tobi-Kadachi S",
            BasariosVambraces => "Brazales Basarios",
            KhezuCoilS => "Faja Khezu S",
            BnahabraGlovesS => "Guantes Bnahabra S",
            DoberCoil => "Faja de Dober",
            BarrothVambraces => "Brazales Barroth",
            SwallowGloves => "Guantes golondrina",
            RathalosBracesS => "Brazales Rathalos S",
            ChannelersHopeS => "Ilusión canalizadora S",
            SinisterSealGarb => "Ropa sello siniestro",
            VolvidonHelmS => "Casco Volvidon S",
            PukeiPukeiHelmS => "Casco Pukei-Pukei S",
            KaiserMail => "Cota Káiser",
            EdelRibplateS => "Armazón Edel S",
            ZinogreGreaves => "Grebas Zinogre",
            KamuraObi => "Obi Kamura",
            KamuraBracesS => "Brazales Kamura S",
            JaggiGauntletsS => "Guanteletes Jaggi S",
            LagombiGreaves => "Grebas Lagombi",
            BishatenMailS => "Cota Bishaten S",
            NargacugaBraces => "Brazales Nargacuga",
            DrothGreavesS => "Grebas Droth S",
            BariothVambraces => "Brazales Barioth",
            JellyGloves => "Guantes gelatina",
            RemobraHeadgear => "Cubrecabeza Remobra",
            GossHaragGreaves => "Grebas Goss Harag",
            MizuhaSleeves => "Mangas Mizuha",
            BarrothMail => "Cota Barroth",
            LeatherBelt => "Muslera de cuero",
            PukeiPukeiCoil => "Faja Pukei-Pukei",
            TetranadonCoil => "Faja Tetranadon",
            MakluvaPants => "Pantalones Makluva",
            TigrexCoil => "Faja Tigrex",
            BoneMailS => "Cota de hueso S",
            BasariosHelmS => "Casco Basarios S",
            BullfangoMaskS => "Máscara Bullfango S",
            MelahoaFolia => "Falda Melahoa",
            RaknaGreaves => "Grebas Rakna",
            VaikGreavesS => "Grebas Vaik S",
            RemobraHeadgearS => "Cubrecabeza Remobra S",
            ArzurosVambraces => "Brazales Arzuros",
            MelahoaJacket => "Chaqueta Melahoa",
            BishatenMail => "Cota Bishaten",
            PukeiPukeiHelm => "Casco Pukei-Pukei",
            UtsushiTassetsV => "Muslera Utsushi (V)",
            DiablosCoil => "Faja Diablos",
            VolvidonGreaves => "Grebas Volvidon",
            DiablosCoilS => "Faja Diablos S",
            UtsushiChestV => "Coraza Utsushi (V)",
            AlmudronVambraces => "Brazales Almudron",
            PukeiPukeiMail => "Cota Pukei-Pukei",
            KadachiGreavesS => "Grebas Kadachi S",
            SpioVertex => "Vértice Spio",
            ChainmailBeltS => "Muslera de malla S",
            IzuchiHelmS => "Casco Izuchi S",
            RathalosMailS => "Cota Rathalos S",
            SinisterSealMask => "Másc. sello siniestro",
            RhenoplosMailS => "Cota Rhenoplos S",
            ChaosArchplate => "Archipeto del caos",
            RhenoplosBracesS => "Brazales Rhenoplos S",
            ChannelersHakama => "Hakama canalizador",
            RhenoplosBraces => "Brazales Rhenoplos",
            TetranadonGreavesS => "Grebas Tetranadon S",
            ZinogreGreavesS => "Grebas Zinogre S",
            DamascusMail => "Cota de Damasco",
            KushalaCocoon => "Capullo Kushala",
            AnjanathHelm => "Casco Anjanath",
            VaikCoil => "Faja Vaik",
            RaknaHelm => "Casco Rakna",
            SpioCrura => "Patas Spio",
            DoberGreaves => "Grebas de Dober",
            NarwasFauld => "Muslera Narwa",
            IngotGreavesS => "Grebas de lingotes S",
            MediumsObiS => "Obi de médium S",
            ShellStuddedVestS => "Chaleco costero S",
            DeathStenchHeel => "Talón pestilente",
            MosgharlVizorS => "Visera Mosgharl S",
            AlmudronGreaves => "Grebas Almudron",
            AknosomGreavesS => "Grebas Aknosom S",
            DrothCoilS => "Faja Droth S",
            AknosomHelm => "Casco Aknosom",
            ShelledSandalsS => "Sandalias costeras S",
            AknosomBraces => "Brazales Aknosom",
            PukeiGreaves => "Grebas Pukei-Pukei",
            BoneCoil => "Faja de hueso",
            DeathStenchBrainS => "Cerebro pestilente S",
            ChannelersObiS => "Obi canalizador S",
            LagombiCoil => "Faja Lagombi",
            AlmudronVambracesS => "Brazales Almudron S",
            BasariosMail => "Cota Basarios",
            AlmudronHelmS => "Casco Almudron S",
            GargwaMaskS => "Máscara de Gargwa S",
            IngotVambraces => "Brazales de lingotes",
            MosgharlRibplate => "Armazón Mosgharl",
            MelahoaRoots => "Raíces Melahoa",
            SinisterSealGreaves => "Grebas sello siniestro",
            GossHaragMail => "Cota Goss Harag",
            NarwasPauldrons => "Hombreras Narwa",
            RathianHelmS => "Yelmo Rathian S",
            KaiserVambraces => "Brazales Káiser",
            RhenoplosMail => "Cota Rhenoplos",
            UtsushiGreavesHS => "Grebas Utsushi (O) S",
            SStuddedSashS => "Fajín costero S",
            KhezuVambracesS => "Brazales Khezu S",
            IzuchiCoil => "Faja Izuchi",
            GossHaragCoil => "Faja Goss Harag",
            RhopessaElytraS => "Élitro Rhopessa S",
            BishatenVambracesS => "Brazales Bishaten S",
            EdelRootsS => "Raíces Edel S",
            TetranadonHelm => "Casco Tetranadon",
            LeatherHeadgear => "Lentes de cuero",
            TobiKadachiBraces => "Brazales Tobi-Kadachi",
            AelucanthBrachiaS => "Ristre Aelucanth S",
            SkaldaVertex => "Vértice Skalda",
            MediumsHakamaS => "Hakama de médium S",
            WroggiMailS => "Cota Wroggi S",
            VaikBraces => "Brazales Vaik",
            RaknaCoil => "Faja Rakna",
            KuluYaKuMail => "Cota Kulu-Ya-Ku",
            BishatenCoil => "Faja Bishaten",
            LudrothCoilS => "Faja Ludroth S",
            UtsushiChestHS => "Coraza Utsushi (O) S",
            ChannelersObi => "Obi canalizador",
            RemobraFeet => "Pezuñas Remobra",
            JaggiMask => "Máscara de Jaggi",
            RhopessaVertex => "Vértice Rhopessa",
            BrigadeCoilS => "Faja de brigada S",
            BoneMail => "Cota de hueso",
            RathianHelm => "Yelmo Rathian",
            LagombiCoilS => "Faja Lagombi S",
            SStuddedSash => "Fajín costero",
            SlagtothCloak => "Capa Slagtoth",
            UroktorCoil => "Faja Uroktor",
            SkaldaThoraxS => "Tórax Skalda S",
            DeathStenchHeelS => "Talón pestilente S",
            KushalaGlare => "Mirada Kushala",
            KhezuGreavesS => "Grebas Khezu S",
            IngotVambracesS => "Brazales de lingotes S",
            RhopessaBrachia => "Ristre Rhopessa",
            MosgharlVizor => "Visera Mosgharl",
            KaiserCoil => "Faja Káiser",
            ChainmailBelt => "Muslera de malla",
            RathianCoilS => "Faja Rathian S",
            ChaosArchbun => "Archimoño del caos",
            MakluvaCoilS => "Cota Makluva S",
            AnjanathGreaves => "Grebas Anjanath",
            MosgharlFoliaS => "Hoja Mosgharl S",
            GoldenHakama => "Hakama dorado",
            MelahoaBranchS => "Rama Melahoa S",
            ArzurosMail => "Cota Arzuros",
            DiablosGreaves => "Grebas Diablos",
            SkaldaElytra => "Élitro Skalda",
            IngotGreaves => "Grebas de lingotes",
            IzuchiGreaves => "Grebas Izuchi",
            AknosomMail => "Cota Aknosom",
            BasariosHelm => "Casco Basarios",
            ZinogreMail => "Cota Zinogre",
            BoneGreavesS => "Grebas de hueso S",
            AnjanathHelmS => "Casco Anjanath S",
            TobiKadachiMail => "Cota Tobi-Kadachi",
            AnjanathCoil => "Faja Anjanath",
            KamuraObiS => "Obi Kamura S",
            MediumsRobeS => "Túnica de médium S",
            RemobraBeltS => "Cinturón Remobra S",
            ArzurosGreaves => "Grebas Arzuros",
            LagombiVambracesS => "Brazales Lagombi S",
            UtsushiGreavesH => "Grebas Utsushi (O)",
            LagombiMail => "Cota Lagombi",
            BariothHelm => "Yelmo Barioth",
            MizutsuneMailS => "Cota Mizutsune S",
            IzuchiMailS => "Cota Izuchi S",
            RhenoplosHelm => "Casco Rhenoplos",
            KuluYaKuGreaves => "Grebas Kulu-Ya-Ku",
            RathalosCoilS => "Faja Rathalos S",
            SinisterGreavesS => "Grebas siniestras S",
            SlagtothHood => "Capucha Slagtoth",
            MakluvaHood => "Capucha Makluva",
            KadachiHelm => "Yelmo Kadachi",
            MizutsuneGreaves => "Grebas Mizutsune",
            BarrothCoilS => "Faja Barroth S",
            MosgharlCreeperS => "Enredo Mosgharl S",
            RhopessaVertexS => "Vértice Rhopessa S",
            HuntersCoilS => "Faja de cazador S",
            RhopessaElytra => "Élitro Rhopessa",
            BoneVambracesS => "Brazales de hueso S",
            VolvidonMail => "Cota Volvidon",
            MosgharlCreeper => "Enredo Mosgharl",
            AknosomBracesS => "Brazales Aknosom S",
            NargacugaHelmS => "Yelmo Nargacuga S",
            KuluYaKuCoilS => "Faja Kulu-Ya-Ku S",
            BrigadeSuitS => "Traje de brigada S",
            RemobraBelt => "Cinturón Remobra",
            VolvidonCoilS => "Faja Volvidon S",
            IngotMailS => "Cota de lingotes S",
            MizuhaGaiters => "Perneras Mizuha",
            BrigadeVambracesS => "Brazales de brigada S",
            MizutsuneHelm => "Casco Mizutsune",
            BarrothGreavesS => "Grebas Barroth S",
            KamuraGarbS => "Vestimenta Kamura S",
            GossHaragGreavesS => "Grebas Goss Harag S",
            BrigadeBootsS => "Botas de brigada S",
            UtsushiMaskH => "Máscara Utsushi (O)",
            RaknaMail => "Cota Rakna",
            KamuraiGarb => "Vestimenta Kamurai",
            TigrexHelmS => "Yelmo Tigrex S",
            RathalosHelm => "Yelmo Rathalos",
            TobiKadachiMailS => "Cota Tobi-Kadachi S",
            ZinogreCoilS => "Faja Zinogre S",
            AelucanthCrura => "Patas Aelucanth",
            TigrexHelm => "Yelmo Tigrex",
            BarrothHelmS => "Yelmo Barroth S",
            DeathStenchMuscle => "Músculo pestilente",
            BasariosMailS => "Cota Basarios S",
            ValstraxHelm => "Casco Valstrax",
            SinisterTassets => "Escarcela siniestra",
            BrigadeSuit => "Traje de brigada",
            LudrothBraces => "Brazales Ludroth",
            BoneVambraces => "Brazales de hueso",
            JellyHat => "Sombrero gelatina",
            MizuhaGuards => "Guardabrazos Mizuha",
            MakluvaCoil => "Cota Makluva",
            MizutsuneCoil => "Faja Mizutsune",
            SpioElytraS => "Élitro Spio S",
            SkaldaVertexS => "Vértice Skalda S",
            KaiserGreaves => "Grebas Káiser",
            SinisterGarb => "Ropaje siniestro",
            ArzurosGreavesS => "Grebas Arzuros S",
            KhezuHelmS => "Casco Khezu S",
            AknosomMailS => "Cota Aknosom S",
            ChannelersHakamaS => "Hakama canalizador S",
            HuntersGreavesS => "Grebas de cazador S",
            UtsushiTassetsH => "Muslera Utsushi (O)",
            AelucanthThorax => "Tórax Aelucanth",
            GoldenHaori => "Haori dorado",
            WroggiVambraces => "Brazales Wroggi",
            SwallowBoots => "Botas golondrina",
            KuluYaKuCoil => "Faja Kulu-Ya-Ku",
            DoberMail => "Cota de Dober",
            AknosomGreaves => "Grebas Aknosom",
            UtsushiBracesHS => "Brazales Utsushi (O) S",
            UtsushiGreavesV => "Grebas Utsushi (V)",
            SinisterHelmS => "Casco siniestro S",
            GossHaragBracesS => "Brazales Goss Harag S",
            KamuraiBraces => "Brazales Kamurai",
            LeatherGloves => "Guantes de cuero",
            GoldenHeaddress => "Tocado dorado",
            RathianMailS => "Cota Rathian S",
            AlmudronCoil => "Faja Almudron",
            ArzurosHelmS => "Casco Arzuros S",
            VolvidonVambraces => "Brazales Volvidon",
            LudrothMail => "Cota Ludroth",
            MelahoaHat => "Sombrero Melahoa",
            MizutsuneHelmS => "Casco Mizutsune S",
            KaiserCrown => "Corona Káiser",
            RaknaArmguards => "Brazos Rakna",
            TetranadonCoilS => "Faja Tetranadon S",
            DrothGreaves => "Grebas Droth",
            KamuraHeadScarfS => "Bandana Kamura S",
            AknosomHelmS => "Casco Aknosom S",
            UtsushiTassetsVS => "Muslera Utsushi (V) S",
            PukeiPukeiMailS => "Cota Pukei-Pukei S",
            KamuraLeggings => "Leotardos Kamura",
            EdelCreeper => "Enredo Edel",
            UtsushiChestVS => "Coraza Utsushi (V) S",
            MizutsuneBracesS => "Brazales Mizutsune S",
            AelucanthBrachia => "Ristre Aelucanth",
            DiablosMail => "Cota Diablos",
            RathalosMail => "Cota Rathalos",
            IbushisBreastplate => "Peto Ibushi",
            LudrothHelm => "Casco Ludroth",
            MediumsRobe => "Túnica de médium",
            DeathStenchBowels => "Intest. pestilente",
            TigrexMailS => "Cota Tigrex S",
            KamuraiObi => "Obi Kamurai",
            IbushisPauldrons => "Hombreras Ibushi",
            NarwasHelm => "Casco Narwa",
            DamascusVambraces => "Brazales de Damasco",
            BoneCoilS => "Faja de hueso S",
            WroggiHelm => "Casco Wroggi",
            SomnacanthHelmS => "Casco Somnacanth S",
            BasariosVambracesS => "Brazales Basarios S",
            TetranadonHelmS => "Casco Tetranadon S",
            EdelRoots => "Raíces Edel",
            VaikHelm => "Casco Vaik",
            BaggiMailS => "Cota Baggi S",
            WroggiGreaves => "Grebas Wroggi",
            Chaoshroom => "Moño del caos",
            BnahabraGloves => "Guantes Bnahabra",
            AelucanthElytra => "Élitro Aelucanth",
            MediumsHairtieS => "Coletero de médium S",
            LudrothMailS => "Cota Ludroth S",
            NargacugaMailS => "Cota Nargacuga S",
            SinisterGarbS => "Ropaje siniestro S",
            KhezuHelm => "Casco Khezu",
            NargacugaHelm => "Yelmo Nargacuga",
            ChainmailVestS => "Cota de malla S",
            SStuddedGlovesS => "Guantes costeros S",
            JaggiShinguards => "Canilleras Jaggi",
            IngotCoil => "Faja de lingotes",
            IzuchiCoilS => "Faja Izuchi S",
            RemobraGlovesS => "Guantes Remobra S",
            JyuratodusCoil => "Faja Jyuratodus",
            GossHaragHelmS => "Casco Goss Harag S",
            AnjanathGreavesS => "Grebas Anjanath S",
            TetranadonBracesS => "Brazales Tetranadon S",
            ChainmailHeadgear => "Yelmo de malla",
            BishatenGreavesS => "Grebas Bishaten S",
            DiablosVambracesS => "Brazales Diablos S",
            BnahabraSuitS => "Traje Bnahabra S",
            ChromeMetalBoots => "Botas metal cromado",
            WroggiVambracesS => "Brazales Wroggi S",
            AknosomCoil => "Faja Aknosom",
            HuntersVambracesS => "Brazales de cazador S",
            MelahoaJacketS => "Chaqueta Melahoa S",
            KhezuCoil => "Faja Khezu",
            LagombiHelm => "Casco Lagombi",
            WroggiCoil => "Faja Wroggi",
            BarrothHelm => "Yelmo Barroth",
            RathianMail => "Cota Rathian",
            BaggiGreaves => "Grebas Baggi",
            MediumsPrayer => "Oración de médium",
            MizutsuneBraces => "Brazales Mizutsune",
            LeatherHeadgearS => "Lentes de cuero S",
            HuntersHelmS => "Casco de cazador S",
            EdelFoliaS => "Hoja Edel S",
            MakluvaPantsS => "Pantalones Makluva S",
            VaikHelmS => "Casco Vaik S",
            BariothCoilS => "Faja Barioth S",
            KhezuGreaves => "Grebas Khezu",
            BariothGreavesS => "Grebas Barioth S",
            MizutsuneGreavesS => "Grebas Mizutsune S",
            JellyGlovesS => "Guantes gelatina S",
            RathianBracesS => "Brazales Rathian S",
            BariothGreaves => "Grebas Barioth",
            SinisterSealBraces => "Brazales sello siniestro",
            BasariosGreavesS => "Grebas Basarios S",
            DiablosHelm => "Yelmo Diablos",
            SinisterGauntletsS => "Guantes siniestros S",
            TetranadonGreaves => "Grebas Tetranadon",
            ChainmailPants => "Pantalones de malla",
            ShellStuddedVest => "Chaleco costero",
            BrigadeBoots => "Botas de brigada",
            RhopessaCruraS => "Patas Rhopessa S",
            SomnacanthBraces => "Brazales Somnacanth",
            MediumsObi => "Obi de médium",
            DeathStenchGripS => "Zarpas pestilentes S",
            SomnacanthCoilS => "Faja Somnacanth S",
            JyuratodusVambraces => "Brazales Jyuratodus",
            VolvidonHelm => "Casco Volvidon",
            MakluvaCoverS => "Poncho Makluva S",
            TigrexBraces => "Brazales Tigrex",
            DStenchMuscleS => "Músculo pestilente S",
            BaggiMail => "Cota Baggi",
            ChromeMetalCoil => "Faja metal cromado",
            ZinogreMailS => "Cota Zinogre S",
            IzuchiGreavesS => "Grebas Izuchi S",
            LeatherVestS => "Chaleco de cuero S",
            BnahabraHatS => "Sombrero Bnahabra S",
            KadachiBracesS => "Brazales Kadachi S",
            SStuddedHatS => "Sombrero costero S",
            ArzurosCoil => "Faja Arzuros",
            RhopessaBrachiaS => "Ristre Rhopessa S",
            BnahabraHat => "Sombrero Bnahabra",
            AlloyMailS => "Cota de aleación S",
            BaggiCoilS => "Faja Baggi S",
            UroktorTorso => "Torso Uroktor",
            AelucanthVertex => "Vértice Aelucanth",
            LeatherBeltS => "Muslera de cuero S",
            SomnacanthHelm => "Casco Somnacanth",
            SkaldaBrachiaS => "Ristre Skalda S",
            RhenoplosGreaves => "Grebas Rhenoplos",
            BoneHelmS => "Yelmo de hueso S",
            SpioBrachia => "Ristre Spio",
            BazelgeuseGreaves => "Grebas Bazelgeuse",
            NargacugaMail => "Cota Nargacuga",
            MelahoaRootsS => "Raíces Melahoa S",
            VolvidonCoil => "Faja Volvidon",
            ChannelersHope => "Ilusión canalizadora",
            TigrexBracesS => "Brazales Tigrex S",
            MelahoaFoliaS => "Falda Melahoa S",
            LudrothCoil => "Faja Ludroth",
            DiablosHelmS => "Yelmo Diablos S",
            DiablosGreavesS => "Grebas Diablos S",
            BrigadeCoil => "Faja de brigada",
            RathalosGreavesS => "Grebas Rathalos S",
            ZinogreHelm => "Yelmo Zinogre",
            KushalaCista => "Quiste Kushala",
            KuluYaKuHelmS => "Casco Kulu-Ya-Ku S",
            ArzurosVambracesS => "Brazales Arzuros S",
            AnjanathMailS => "Cota Anjanath S",
            KhezuVambraces => "Brazales Khezu",
            JyuratodusMail => "Cota Jyuratodus",
            SinisterSealObi => "Obi sello siniestro",
            BoneGreaves => "Grebas de hueso",
            AnjanathMail => "Cota Anjanath",
            CunningSpecs => "Anteojos astutos",
            SkaldaCrura => "Patas Skalda",
            MizuhaSash => "Fajín Mizuha",
            BarrothVambracesS => "Brazales Barroth S",
            VolvidonVambracesS => "Brazales Volvidon S",
            GoldenObi => "Obi dorado",
            DrothMail => "Cota Droth",
            DeathStenchGrip => "Zarpas pestilentes",
            NargacugaGreaves => "Grebas Nargacuga",
            TetranadonMailS => "Cota Tetranadon S",
            RathianCoil => "Faja Rathian",
            BariothVambracesS => "Brazales Barioth S",
            HuntersMail => "Cota de cazador",
            SStuddedHat => "Sombrero costero",
            ArzurosCoilS => "Faja Arzuros S",
            IngotHelmS => "Yelmo de lingotes S",
            JyuratodusGreaves => "Grebas Jyuratodus",
            BrigadeLobos => "Sombrero de brigada",
            TetranadonBraces => "Brazales Tetranadon",
            TigrexCoilS => "Faja Tigrex S",
            AlloyMail => "Cota de aleación",
            BarrothMailS => "Cota Barroth S",
            AelucanthThoraxS => "Tórax Aelucanth S",
            KamuraiHat => "Sombrero Kamurai",
            BullfangoMask => "Máscara Bullfango",
            VaikBracesS => "Brazales Vaik S",
            KuluYaKuMailS => "Cota Kulu-Ya-Ku S",
            KhezuMailS => "Cota Khezu S",
            RemobraFeetS => "Pezuñas Remobra S",
            ChannelersRobe => "Atuendo canalizador",
            SkaldaCruraS => "Patas Skalda S",
            PukeiGreavesS => "Grebas Pukei-Pukei S",
            SpioThoraxS => "Tórax Spio S",
            PukeiPukeiBracesS => "Brazales Pukei-Pukei S",
            KamuraHeadScarf => "Bandana Kamura",
            TigrexGreaves => "Grebas Tigrex",
            EdelVizor => "Visera Edel",
            SpioVertexS => "Vértice Spio S",
            ValstraxMail => "Cota Valstrax",
            BaggiCoil => "Faja Baggi",
            MizutsuneMail => "Cota Mizutsune",
            BazelgeuseMail => "Cota Bazelgeuse",
            SpioThorax => "Tórax Spio",
            BishatenCoilS => "Faja Bishaten S",
            MizuhaCap => "Gorro Mizuha",
            ChainmailPantsS => "Pantalones de malla S",
            JellyVestS => "Chaleco gelatina S",
            UtsushiBracesH => "Brazales Utsushi (O)",
            RhenoplosCoil => "Faja Rhenoplos",
            KushalaGrip => "Agarres Kushala",
            BazelgeuseBraces => "Brazales Bazelgeuse",
            AlloyVambraces => "Brazales de aleación",
            BasariosCoil => "Faja Basarios",
            SinisterGauntlets => "Guantes siniestros",
            ChainmailGlovesS => "Guantes de malla S",
            NarwaBreastplate => "Peto Narwa",
            UroktorCoilS => "Faja Uroktor S",
            FeatherofMastery => "Pluma de maestría",
            KuluYaKuHelm => "Casco Kulu-Ya-Ku",
            AlmudronMail => "Cota Almudron",
            JellyHatS => "Sombrero gelatina S",
            SomnacanthMail => "Cota Somnacanth",
            EdelRibplate => "Armazón Edel",
            KamuraiLeggings => "Leotardos Kamurai",
            SpioCruraS => "Patas Spio S",
            BishatenHelm => "Casco Bishaten",
            SwallowShirt => "Camisa golondrina",
            AknosomCoilS => "Faja Aknosom S",
            MakluvaSleeves => "Mangas Makluva",
            ChannelersHairtie => "Coletero canalizador",
            VaikMailS => "Cota Vaik S",
            MakluvaHoodS => "Capucha Makluva S",
            GargwaMask => "Máscara de Gargwa",
            MosgharlRibplateS => "Armazón Mosgharl S",
            GossHaragMailS => "Cota Goss Harag S",
            DiablosMailS => "Cota Diablos S",
            TobiKadachiCoil => "Faja Tobi-Kadachi",
            JyuratodusHelm => "Casco Jyuratodus",
            SkaldaThorax => "Tórax Skalda",
            RhenoplosCoilS => "Faja Rhenoplos S",
            KadachiGreaves => "Grebas Kadachi",
            LudrothGreaves => "Grebas Ludroth",
            FlameSeal => "Sello ígneo",
            DamascusHelm => "Yelmo de Damasco",
            HuntersCoil => "Faja de cazador",
            LeatherPantsS => "Pantalones de cuero S",
            RhenoplosGreavesS => "Grebas Rhenoplos S",
            VaikCoilS => "Faja Vaik S",
            BishatenHelmS => "Casco Bishaten S",
            PukeiPukeiBraces => "Brazales Pukei-Pukei",
            BasariosGreaves => "Grebas Basarios",
            SomnacanthGreaves => "Grebas Somnacanth",
            LudrothGreavesS => "Grebas Ludroth S",
            AnjanathVambraces => "Brazales Anjanath",
            BariothHelmS => "Casco Barioth S",
            DamascusCoil => "Faja de Damasco",
            BnahabraBoots => "Botas Bnahabra",
            BishatenVambraces => "Brazales Bishaten",
            SlagtothCloakS => "Capa Slagtoth S",
            FootofNarwa => "Pie Narwa",
            ZinogreBracesS => "Brazales Zinogre S",
            UtsushiBracesV => "Brazales Utsushi (V)",
            AlloyHelmS => "Yelmo de aleación S",
            FoxMask => "Máscara de zorro",
            DeathStenchBrain => "Cerebro pestilente",
            NargacugaCoilS => "Faja Nargacuga S",
            AelucanthVertexS => "Vértice Aelucanth S",
            RathalosHelmS => "Yelmo Rathalos S",
            DamascusGreaves => "Grebas de Damasco",
            EdelFolia => "Hoja Edel",
            KamuraBraces => "Brazales Kamura",
            MosgharlFolia => "Hoja Mosgharl",
            BariothCoil => "Faja Barioth",
            ArzurosMailS => "Cota Arzuros S",
            HuntersVambraces => "Brazales de cazador",
            UtsushiMaskHS => "Máscara Utsushi (O) S",
            LagombiVambraces => "Brazales Lagombi",
            RathalosGreaves => "Grebas Rathalos",
            RemobraGloves => "Guantes Remobra",
            MosgharlRootsS => "Raíces Mosgharl S",
            BazelgeuseHelm => "Casco Bazelgeuse",
            ZinogreCoil => "Faja Zinogre",
            UtsushiGreavesVS => "Grebas Utsushi (V) S",
            HuntersHelm => "Casco de cazador",
            SomnacanthBracesS => "Brazales Somnacanth S",
            JaggiGauntlets => "Guanteletes Jaggi",
            BnahabraSuit => "Traje Bnahabra",
            JaggiShinguardsS => "Canilleras Jaggi S",
            SpioBrachiaS => "Ristre Spio S",
            IzuchiBracesS => "Brazales Izuchi S",
            ValstraxGreaves => "Grebas Valstrax",
            BishatenGreaves => "Grebas Bishaten",
            UtsushiMaskVS => "Máscara Utsushi (V) S",
            BnahabraCoilS => "Faja Bnahabra S",
            JellyBootsS => "Botas gelatina S",
            TheaterWig => "Peluca teatral",
            ZinogreHelmS => "Yelmo Zinogre S",
            IngotCoilS => "Faja de lingotes S",
            TigrexMail => "Cota Tigrex",
            BoneHelm => "Yelmo hueso",
            IngotMail => "Cota de lingotes",
            ShellStuddedGloves => "Guantes costeros",
            HuntersGreaves => "Grebas de cazador",
            RhopessaCrura => "Patas Rhopessa",
            ChainmailHeadgearS => "Yelmo de malla S",
            LeatherPants => "Pantalones de cuero",
            RathianGreaves => "Grebas Rathian",
            AlloyGreavesS => "Grebas de aleación S",
            WroggiHelmS => "Casco Wroggi S",
            BaggiHelmS => "Casco Baggi S",
            JaggiMaskS => "Máscara de Jaggi S",
            DStenchBowelsS => "Intest. pestilente S",
            AelucanthElytraS => "Élitro Aelucanth S",
            ChainmailGloves => "Guantes de malla",
            LeatherGlovesS => "Guantes de cuero S",
            AnjanathCoilS => "Faja Anjanath S",
            VolvidonGreavesS => "Grebas Volvidon S",
            MightyBowFeather => "Pluma arco poderoso",
            LagombiMailS => "Cota Lagombi S",
            FootofIbushi => "Pie Ibushi",
            KadachiHelmS => "Yelmo Kadachi S",
            WroggiCoilS => "Faja Wroggi S",
            BaggiGreavesS => "Grebas Baggi S",
            LagombiHelmS => "Casco Lagombi S",
            BrigadeVambraces => "Brazales de brigada",
            RemobraSuit => "Traje Remobra",
            BarrothGreaves => "Grebas Barroth",
            ZinogreBraces => "Brazales Zinogre",
            GossHaragBraces => "Brazales Goss Harag",
            JellyCoil => "Cinto gelatina",
            RemobraSuitS => "Traje Remobra S",
            EdelCreeperS => "Enredo Edel S",
            SinisterGreaves => "Grebas siniestras",
            EdelVizorS => "Visera Edel S",
            ChannelersHairtieS => "Coletero canalizador S",
            SkaldaElytraS => "Élitro Skalda S",
            ShelledSandals => "Sandalias costeras",
            LudrothBracesS => "Brazales Ludroth S",
            KuluYaKuBracesS => "Brazales Kulu-Ya-Ku S",
            UroktorTorsoS => "Torso Uroktor S",
            UtsushiMaskV => "Máscara Utsushi (V)",
            IzuchiBraces => "Brazales Izuchi",
            BnahabraCoil => "Faja Bnahabra",
            AlloyHelm => "Yelmo de aleación",
            AlmudronHelm => "Casco Almudron",
            AnjanathVambracesS => "Brazales Anjanath S",
            BazelgeuseCoil => "Faja Bazelgeuse",
            KushalaCrus => "Perneras Kushala",
            LagombiGreavesS => "Grebas Lagombi S",
            AlloyGreaves => "Grebas de aleación",
            JellyVest => "Chaleco gelatina",
            BaggiVambraces => "Brazales Baggi",
            MediumsHakama => "Hakama de médium",
            SinisterTassetsS => "Escarcela siniestra S",
            SlagtothHoodS => "Capucha Slagtoth S",
            DiablosVambraces => "Brazales Diablos",
            LeatherVest => "Chaleco de cuero",
            AlloyCoil => "Faja de aleación",
            UtsushiChestH => "Coraza Utsushi (O)",
            AlmudronCoilS => "Faja Almudron S",
            KhezuMail => "Cota Khezu",
            MediumsPrayerS => "Oración de médium S",
            RhopessaThoraxS => "Tórax Rhopessa S",
            AlmudronGreavesS => "Grebas Almudron S",
            ChainmailVest => "Cota de malla",
            NargacugaBracesS => "Brazales Nargacuga S",
            AelucanthCruraS => "Patas Aelucanth S",
            ChaosPlate => "Peto del caos",
            HuntersMailS => "Cota de cazador S",
            RathalosBraces => "Brazales Rathalos",
            BaggiVambracesS => "Brazales Baggi S",
            RathalosCoil => "Faja Rathalos",
            SkullVisage => "Rostro calavérico",
            ChannelersRobeS => "Atuendo canalizador S",
            RathianBraces => "Brazales Rathian",
            VaikGreaves => "Grebas Vaik",
            IzuchiMail => "Cota Izuchi",
            MediumsHairtie => "Coletero de médium",
            IzuchiHelm => "Casco Izuchi",
            SomnacanthCoil => "Faja Somnacanth",
            RhenoplosHelmS => "Casco Rhenoplos S",
            BasariosCoilS => "Faja Basarios S",
        }
    }
}

impl Spanish for super::UiSymbole {
    fn to_spanish(&self) -> &'static str {
        super::en::English::to_english(self)
    }
}
