pub trait Polish {
    fn to_polish(&self) -> &'static str;
}

impl Polish for crate::DisplaySkill {
    fn to_polish(&self) -> &'static str {
        use rab_core::armor_and_skills::Skill::*;

        match self.0 {
            HornMaestro => "Wirtuoz rogu",
            SpecialAmmoBoost => "Moc amun. specjal.",
            DefenseBoost => "Wzmocnienie obrony",
            CarvingPro => "Spec od wycinania",
            CriticalEye => "Krytyczne oko",
            Windproof => "Wiatrochron",
            FireResistance => "Odporn. na ogień",
            Resentment => "Uraza",
            PunishingDraw => "Mordercze dobycie",
            RazorSharp => "Ostry jak brzytwa",
            LoadShells => "Ładuj łuski",
            SpeedEating => "Szybkie jedzenie",
            ThunderAttack => "Atak piorunem",
            PoisonResistance => "Odporn. na truciznę",
            Botanist => "Botanik",
            CarvingMaster => "Mistrz Wykrajania",
            ChameleosBlessing => "Błogosł. Chameleosa",
            WindAlignment => "Wyrównanie wiatru",
            MaximumMight => "Maksymalna moc",
            Dragonheart => "Smocze serce",
            BlastResistance => "Odporność na wybuch",
            Stormsoul => "Burzodusza",
            WirebugWhisperer => "Zaklinacz kablobaków",
            HellfireCloak => "Piekielna peleryna",
            MuckResistance => "Odpor. na wydzieliny",
            EvadeWindow => "Czas na unik",
            ThunderAlignment => "Wyrównanie pioruna",
            GoodLuck => "Szczęście",
            Partbreaker => "Rozłamywacz",
            Agitator => "Podżegacz",
            RecoveryUp => "Wzrost regeneracji",
            ProtectivePolish => "Lakier ochronny",
            MindsEye => "Oko umysłu",
            IceAttack => "Atak lodem",
            NormalRapidUp => "Więcej normal./szyb.",
            CriticalBoost => "Wzmocnienie krytyczne",
            AmmoUp => "Więcej amunicji",
            RecoverySpeed => "Szybkość regeneracji",
            Resuscitate => "Ożywienie",
            WaterResistance => "Odporn. na wodę",
            ThunderResistance => "Odporn. na pioruny",
            TeostraBlessing => "Błogosław. Teostry",
            PowerProlonger => "Przedłużenie mocy",
            SpreadUp => "Więcej rozrzutowej",
            RapidMorph => "Szybka transformacja",
            Ballistics => "Balistyka",
            FlinchFree => "Kamienna twarz",
            ParalysisAttack => "Atak paraliżem",
            OffensiveGuard => "Garda ofensywna",
            Earplugs => "Zatyczki do Uszu",
            MastersTouch => "Dotyk mistrza",
            Heroics => "Wyczyny",
            ItemProlonger => "Długie trwanie",
            Steadiness => "Stałość",
            Guard => "Garda",
            FreeMeal => "Darmowy posiłek",
            CaptureMaster => "Mistrz łapania",
            DivineBlessing => "Boskie błogosławień.",
            AttackBoost => "Wzmocnienie ataku",
            CriticalDraw => "Dobycie krytyczne",
            RecoilDown => "Mniejszy odrzut",
            DragonResistance => "Odporn. na smoki",
            QuickSheath => "Szybkie chowanie",
            Slugger => "Powalacz",
            BlightResistance => "Odporność na plagę",
            Focus => "Skupienie",
            GuardUp => "Wzniesiona garda",
            ParalysisResistance => "Odporność na paraliż",
            StaminaThief => "Kradzież wytrzymał.",
            StunResistance => "Odpor. na ogłuszenie",
            WideRange => "Duży zasięg",
            JumpMaster => "Mistrz skakania",
            HungerResistance => "Odporność na głód",
            FireAttack => "Atak ogniem",
            WallRunner => "Ścianobiegacz",
            Handicraft => "Rękodzieło",
            LatentPower => "Uśpiona moc",
            ReloadSpeed => "Szybk. przeładowania",
            CriticalElement => "Żywioł krytyczny",
            WaterAttack => "Atak wodą",
            Bludgeoner => "Pałkarz",
            SleepAttack => "Atak uśpieniem",
            PierceUp => "Więcej przebijającej",
            IceResistance => "Odporn. na lód",
            SleepResistance => "Odporn. na uśpienie",
            SpareShot => "Zapasowy strzał",
            PeakPerformance => "Szczyt możliwości",
            Geologist => "Geolog",
            Fortify => "Wzmocnienie",
            KushalaBlessing => "Błogosław. Kushali",
            BlastAttack => "Atak wybuchem",
            TremorResistance => "Odporn. na wstrząsy",
            BowChargePlus => "Lepsze ładow. łuku",
            RapidFireUp => "Więcej szybk. ognia",
            Diversion => "Dywersja",
            StaminaSurge => "Przypływ wytrzym.",
            DragonAttack => "Smoczy atak",
            Counterstrike => "Kontratak",
            AffinitySliding => "Wślizg zgodności",
            BubblyDance => "Bąblowy taniec",
            SpeedSharpening => "Szybkie ostrzenie",
            Constitution => "Tężyzna",
            Bombardier => "Bombardier",
            Artillery => "Artyleria",
            PoisonAttack => "Atak trucizną",
            Mushroomancer => "Grzybomanta",
            MarathonRunner => "Maratończyk",
            EvadeExtender => "Wydłużenie uniku",
            MasterMounter => "Mistrz dosiadania",
            LeapofFaith => "Skok wiary",
            WeaknessExploit => "Wykorzyst. słabości",
        }
    }
}

impl Polish for crate::armors::Armor {
    fn to_polish(&self) -> &'static str {
        use crate::armors::Armor::*;

        match self {
            Chaoshroom => "Grzyb Chaosu",
            MediumsHairtieS => "Opaska medium S",
            MizutsuneMail => "Zbroja z Mizutsune",
            SinisterSealObi => "Obi złowrogiej pieczęci",
            IzuchiGreavesS => "Nagol. z Izuchiego S",
            KhezuGreavesS => "Nagolenice z Khezu S",
            IbushisBreastplate => "Napierś. z Ibushiego",
            MakluvaCoilS => "Pas Makluva S",
            AlmudronHelm => "Hełm z Almudrona",
            GossHaragHelmS => "Hełm z Goss Haraga S",
            PukeiPukeiHelmS => "Hełm z Pukei-Pukei S",
            BoneGreavesS => "Kościane nagol. S",
            KamuraObi => "Obi Kamury",
            BoneMailS => "Kościana zbroja S",
            ArzurosGreavesS => "Nagol. z Arzurosa S",
            JellyHat => "Kapelusz Jelly",
            KamuraHeadScarf => "Chustka Kamury",
            RathalosCoilS => "Pas z Rathalosa S",
            MelahoaRoots => "Korzenie Melahoa",
            UroktorTorso => "Napierś. z Uroktora",
            TetranadonBracesS => "Zaręk. z Tetranad. S",
            RemobraBelt => "Pas z Remobry",
            WroggiCoilS => "Pas z Wroggiego S",
            BaggiHelmS => "Hełm z Baggiego S",
            BrigadeSuitS => "Strój Brygady S",
            SpioThoraxS => "Tułów Spio S",
            DStenchMuscleS => "Mięsień rozkładu S",
            KaiserCrown => "Korona kajzera",
            BasariosMailS => "Zbroja z Basariosa S",
            DamascusVambraces => "Zaręk. damasceńskie",
            SkaldaThorax => "Tułów Skaldy",
            AelucanthElytra => "Okrywy Aelucantha",
            JyuratodusMail => "Zbr. z Jyuratodusa",
            AelucanthThorax => "Tułów Aelucantha",
            RhenoplosBracesS => "Zaręk. z Rhenopl. S",
            KuluYaKuHelm => "Hełm z Kulu-Ya-Ku",
            RathianMailS => "Zbroja z Rathiany S",
            RhopessaBrachia => "Ramiona Rhopessy",
            DeathStenchGrip => "Chwyt rozkładu",
            RathianMail => "Zbroja z Rathiany",
            RathianCoil => "Pas z Rathiany",
            BrigadeLobos => "Kapelusz Brygady",
            LagombiCoil => "Pas z Lagombiego",
            AlloyMailS => "Zbroja ze stopu S",
            BnahabraBoots => "Buty z Bnahabry",
            GoldenObi => "Złote obi",
            PukeiPukeiHelm => "Hełm z Pukei-Pukei",
            EdelCreeper => "Pnącze Edel",
            GossHaragBraces => "Zaręk. z Goss Haraga",
            KhezuHelm => "Hełm z Khezu",
            NargacugaMail => "Zbroja z Nargacuga",
            KamuraLeggings => "Nogawice Kamury",
            BishatenGreaves => "Nagol. z Bishatena",
            SwallowGloves => "Jaskółcze rękawice",
            ShellStuddedGloves => "Muszlowe rękawice",
            RemobraSuit => "Strój z Remobry",
            HuntersMailS => "Zbroja łowcy S",
            MizuhaSash => "Szarfa Mizuha",
            LudrothCoil => "Pas z Ludrotha",
            WroggiMailS => "Zbroja z Wroggiego S",
            LagombiGreavesS => "Nag. z Lagombiego S",
            DamascusHelm => "Hełm damasceński",
            ChromeMetalBoots => "Chromowe buty",
            KushalaCrus => "Nogi Kushali",
            SinisterGarb => "Złowrogi strój",
            BasariosHelmS => "Hełm z Basariosa S",
            RathalosBracesS => "Zaręk. z Rathalosa S",
            ChainmailGloves => "Rękawice kolcze",
            ChannelersHope => "Nadzieja pośrednika",
            BishatenHelm => "Hełm z Bishatena",
            HuntersCoilS => "Pas łowcy S",
            UroktorTorsoS => "Napierś. z Urokt. S",
            VaikHelmS => "Hełm Vaik S",
            LeatherPants => "Skórzane spodnie",
            MediumsPrayerS => "Modlitwa medium S",
            KaiserGreaves => "Nagolenice kajzera",
            PukeiGreavesS => "Nagolenice z Pukei S",
            KadachiBracesS => "Zaręk. z Kadachi S",
            KhezuVambracesS => "Zarękawia z Khezu S",
            BishatenCoilS => "Pas z Bishatena S",
            ChannelersHairtie => "Opaska pośrednika",
            VolvidonGreavesS => "Nagol. z Volvidona S",
            MizutsuneBracesS => "Zaręk. z Mizutsune S",
            GoldenHakama => "Złota hakama",
            ArzurosMailS => "Zbroja z Arzurosa S",
            SkaldaVertex => "Czub Skaldy",
            BariothVambracesS => "Zaręk. z Bariotha S",
            ChainmailVestS => "Kolczuga S",
            TobiKadachiCoilS => "Pas z Tobi-Kadachi S",
            VaikMailS => "Zbroja Vaik S",
            RaknaMail => "Zbroja z Rakny",
            IngotHelm => "Hełm ze sztabek",
            UtsushiBracesVS => "Zaręk. Utsush. (V) S",
            RathalosMailS => "Zbroja z Rathalosa S",
            VolvidonVambraces => "Zaręk. z Volvidona",
            ZinogreHelmS => "Hełm z Zinogra S",
            KadachiGreavesS => "Nagolen. z Kadachi S",
            MizutsuneBraces => "Zaręk. z Mizutsune",
            RemobraHeadgear => "Hełm z Remobry",
            VolvidonHelm => "Hełm z Volvidona",
            ArzurosCoilS => "Pas z Arzurosa S",
            ValstraxBraces => "Zaręk. z Valstraxa",
            ValstraxGreaves => "Nagol. z Valstraxa",
            JellyHatS => "Kapelusz Jelly S",
            KamuraiGarb => "Szata Kamuraj",
            SlagtothCloak => "Płaszcz ze Slagtotha",
            BnahabraHat => "Kapelusz z Bnahabry",
            ArzurosHelmS => "Hełm z Arzurosa S",
            ChannelersObiS => "Amulet pośrednika S",
            UtsushiBracesHS => "Zaręk. Utsush. (H) S",
            KaiserCoil => "Pas kajzera",
            HuntersVambraces => "Zarękawia łowcy",
            MelahoaFolia => "Listowie Melahoa",
            VaikCoilS => "Pas Vaik S",
            BnahabraBootsS => "Buty z Bnahabry S",
            BarrothVambraces => "Zarękawia z Barrotha",
            RhenoplosMailS => "Zbr. z Rhenoplosa S",
            UtsushiMaskVS => "Maska Utsush. (V) S",
            BarrothHelmS => "Hełm z Barrotha S",
            KamuraLeggingsS => "Nogawice Kamury S",
            BasariosCoil => "Pas z Basariosa",
            RathalosHelm => "Hełm z Rathalosa",
            MakluvaCoil => "Pas Makluva",
            SinisterSealGreaves => "Nagol. złowr. pieczęci",
            UtsushiTassetsV => "Taszki Utsush. (V)",
            LagombiHelmS => "Hełm z Lagombiego S",
            GargwaMask => "Maska z Gargwy",
            KamuraBraces => "Zarękawia Kamury",
            DStenchBowelsS => "Jelita rozkładu S",
            RaknaArmguards => "Zarękawia z Rakny",
            AlloyCoil => "Pas ze stopu",
            NargacugaCoil => "Pas z Nargacuga",
            IngotHelmS => "Hełm ze sztabek S",
            BarrothVambracesS => "Zaręk. z Barrotha S",
            RhopessaCruraS => "Nogi Rhopessy S",
            AlmudronMailS => "Zbroja z Almudrona S",
            AlmudronCoilS => "Pas z Almudrona S",
            ZinogreMailS => "Zbroja z Zinogra S",
            DoberCoil => "Pas z Dobera",
            SinisterGauntletsS => "Złowrogie rękawice S",
            NargacugaBraces => "Zaręk. z Nargacuga",
            DiablosCoilS => "Pas z Diablosa S",
            SinisterTassets => "Złowrogie nabiodrki",
            ValstraxHelm => "Hełm z Valstraxa",
            BaggiHelm => "Hełm z Baggiego",
            SkaldaElytraS => "Okrywy Skaldy S",
            JaggiGauntlets => "Rękawice z Jaggiego",
            VaikMail => "Zbroja Vaik",
            SpioCruraS => "Nogi Spio S",
            KhezuMailS => "Zbroja z Khezu S",
            BishatenMailS => "Zbroja z Bishatena S",
            DiablosCoil => "Pas z Diablosa",
            RathalosHelmS => "Hełm z Rathalosa S",
            GossHaragMailS => "Zbr. z Goss Haraga S",
            BariothGreaves => "Nagol. z Bariotha",
            LudrothBracesS => "Zaręk. z Ludrotha S",
            VolvidonGreaves => "Nagol. z Volvidona",
            TetranadonGreaves => "Nag. z Tetranadona",
            JellyVest => "Kamizelka Jelly",
            LeatherBeltS => "Skórzany pas S",
            IngotVambraces => "Zarękawia ze sztabek",
            BasariosVambracesS => "Zaręk. z Basariosa S",
            PukeiPukeiMailS => "Zbr. z Pukei-Pukei S",
            KamuraGarbS => "Strój Kamury S",
            AelucanthVertexS => "Czub Aelucantha S",
            IbushisFauld => "Fartuch z Ibushiego",
            AlmudronCoil => "Pas z Almudrona",
            NargacugaGreavesS => "Nagol. z Nargacuga S",
            FlameSeal => "Płomienna Pieczęć",
            JellyGloves => "Rękawiczki Jelly",
            ArzurosCoil => "Pas z Arzurosa",
            UtsushiTassetsVS => "Taszki Utsush. (V) S",
            ChannelersRobeS => "Szata pośrednika S",
            SomnacanthBracesS => "Zaręk. z Somnacan. S",
            SkullVisageS => "Oblicze czaszki S",
            AelucanthVertex => "Czub Aelucantha",
            ArzurosVambraces => "Zarękawia z Arzurosa",
            SpioVertexS => "Czub Spio S",
            MosgharlRibplateS => "Napierś. Mosgharl S",
            BishatenGreavesS => "Nagol. z Bishatena S",
            SinisterGreavesS => "Złowrogie nagol. S",
            LeatherPantsS => "Skórzane spodnie S",
            WroggiHelmS => "Hełm z Wroggiego S",
            LeatherBelt => "Skórzany pas",
            HuntersHelm => "Hełm łowcy",
            IzuchiMail => "Zbroja z Izuchiego",
            LagombiHelm => "Hełm z Lagombiego",
            SpioCrura => "Nogi Spio",
            VolvidonCoil => "Pas z Volvidona",
            RhopessaVertex => "Czub Rhopessy",
            BarrothGreavesS => "Nagol. z Barrotha S",
            DoberVambraces => "Zarękawia z Dobera",
            JyuratodusGreaves => "Nagol. z Jyuratodusa",
            ZinogreBracesS => "Zaręk. z Zinogra S",
            KuluYaKuBracesS => "Zar. z Kulu-Ya-Ku S",
            BarrothMail => "Zbroja z Barrotha",
            BoneHelm => "Kościany hełm",
            JaggiMaskS => "Maska z Jaggiego S",
            SStuddedHatS => "Gwiezdny kapelusz S",
            MelahoaBranch => "Gałąź Melahoa",
            RhenoplosGreaves => "Nagol. z Rhenoplosa",
            RhopessaThoraxS => "Tułów Rhopessy S",
            GossHaragGreavesS => "Nag. z Goss Haraga S",
            SStuddedGlovesS => "Muszlowe rękawice S",
            GossHaragHelm => "Hełm z Goss Haraga",
            SlagtothCloakS => "Płaszcz ze Slagt. S",
            HuntersVambracesS => "Zarękawia łowcy S",
            MelahoaJacketS => "Kubrak Melahoa S",
            SomnacanthGreavesS => "Nag. z Somnacantha S",
            KadachiHelmS => "Hełm z Kadachi S",
            TobiKadachiBraces => "Zar. z Tobi-Kadachi",
            LeatherHeadgearS => "Skórzany kaptur S",
            AknosomGreaves => "Nagol. z Aknosoma",
            RaknaHelm => "Hełm z Rakny",
            RemobraGloves => "Rękawice z Remobry",
            RathalosGreaves => "Nagol. z Rathalosa",
            ArzurosMail => "Zbroja z Arzurosa",
            NargacugaGreaves => "Nagol. z Nargacuga",
            AelucanthCrura => "Nogi Aelucantha",
            RathianGreavesS => "Nagol. z Rathiany S",
            AelucanthCruraS => "Nogi Aelucantha S",
            SkaldaCruraS => "Nogi Skaldy S",
            RathianHelmS => "Hełm z Rathiany S",
            AlloyHelm => "Hełm ze stopu",
            IzuchiCoil => "Pas z Izuchiego",
            MakluvaPantsS => "Spodnie Makluva S",
            MizutsuneGreaves => "Nag. z Mizutsune",
            RathalosGreavesS => "Nagol. z Rathalosa S",
            SStuddedSash => "Gwiezdna szarfa",
            BnahabraGloves => "Rękawice z Bnahabry",
            RathianHelm => "Hełm z Rathiany",
            RathalosMail => "Zbroja z Rathalosa",
            VolvidonHelmS => "Hełm z Volvidona S",
            ChainmailHeadgear => "Hełm kolczy",
            TetranadonMail => "Zbr. z Tetranadona",
            UtsushiGreavesHS => "Nagol. Utsush. (H) S",
            BnahabraCoil => "Pas z Bnahabry",
            RaknaCoil => "Pas z Rakny",
            MakluvaSleevesS => "Rękawy Makluva S",
            AlmudronVambraces => "Zaręk. z Almudrona",
            MizutsuneGreavesS => "Nag. z Mizutsune S",
            IngotGreaves => "Nagol. ze sztabek",
            AlloyCoilS => "Pas ze stopu S",
            BnahabraCoilS => "Pas z Bnahabry S",
            DeathStenchHeelS => "Pięta rozkładu S",
            AnjanathCoil => "Pas z Anjanatha",
            TigrexHelmS => "Hełm z Tigrexa S",
            LudrothGreaves => "Nagol. z Ludrotha",
            WroggiVambracesS => "Zaręk. z Wroggiego S",
            IngotMailS => "Zbroja ze sztabek S",
            BishatenMail => "Zbroja z Bishatena",
            BoneHelmS => "Kościany hełm S",
            KushalaCocoon => "Kokon Kushali",
            MediumsRobeS => "Szata medium S",
            TigrexCoil => "Pas z Tigrexa",
            BariothHelm => "Hełm z Bariotha",
            AknosomGreavesS => "Nagol. z Aknosoma S",
            UtsushiGreavesV => "Nagolen. Utsush. (V)",
            PukeiPukeiBraces => "Zaręk. z Pukei-Pukei",
            JellyGlovesS => "Rękawiczki Jelly S",
            BrigadeCoilS => "Pas Brygady S",
            MediumsObiS => "Amulet medium S",
            KushalaGlare => "Spojrzenie Kushali",
            TetranadonMailS => "Zbr. z Tetranad. S",
            ChromeMetalCoil => "Chromowany pas",
            AnjanathMailS => "Zbroja z Anjanatha S",
            UtsushiGreavesH => "Nagolen. Utsush. (H)",
            AnjanathCoilS => "Pas z Anjanatha S",
            FootofIbushi => "Stopa Ibushiego",
            BariothGreavesS => "Nagol. z Bariotha S",
            DiablosGreaves => "Nagol. z Diablosa",
            LeatherVestS => "Kamizela skórzana S",
            HuntersGreaves => "Nagolenice łowcy",
            SomnacanthHelm => "Hełm z Somnacantha",
            JellyVestS => "Kamizelka Jelly S",
            TetranadonCoil => "Pas z Tetranadona",
            SomnacanthHelmS => "Hełm z Somnacantha S",
            BaggiMailS => "Zbroja z Baggiego S",
            MelahoaRootsS => "Korzenie Melahoa S",
            KuluYaKuBraces => "Zaręk. z Kulu-Ya-Ku",
            UtsushiGreavesVS => "Nagol. Utsush. (V) S",
            BishatenHelmS => "Hełm z Bishatena S",
            TigrexBracesS => "Zaręk. z Tigrexa S",
            SkaldaElytra => "Okrywy Skaldy",
            RemobraFeetS => "Stopy Remobry S",
            MakluvaPants => "Spodnie Makluva",
            AelucanthBrachia => "Ramiona Aelucantha",
            RathalosBraces => "Zaręk. z Rathalosa",
            LudrothHelmS => "Hełm z Ludrotha S",
            AknosomBraces => "Zarękawia z Aknosoma",
            JaggiGauntletsS => "Rękawice z Jagg. S",
            MosgharlVizor => "Przyłbica Mosgharl",
            KushalaCista => "Zbroja z Kushali",
            KaiserVambraces => "Zarękawia kajzera",
            KuluYaKuMailS => "Zbr. z Kulu-Ya-Ku S",
            DrothGreavesS => "Nagol. z Drotha S",
            NargacugaCoilS => "Pas z Nargacuga S",
            BazelgeuseGreaves => "Nagol. z Bazelgeza",
            MediumsRobe => "Szata medium",
            TigrexBraces => "Zarękawia z Tigrexa",
            MizuhaGuards => "Napierśnik Mizuha",
            MakluvaSleeves => "Rękawy Makluva",
            VaikBraces => "Zarękawia Vaik",
            MediumsObi => "Amulet medium",
            SinisterSealBraces => "Zaręk. złowr. pieczęci",
            IngotMail => "Zbroja ze sztabek",
            BullfangoMaskS => "Maska Bullfango S",
            ChaosArchbun => "Arcykoczek Chaosu",
            KamuraiObi => "Obi Kamuraj",
            SStuddedHat => "Gwiezdny kapelusz",
            MosgharlCreeperS => "Pnącze Mosgharl S",
            KamuraiBraces => "Zarękawia Kamuraj",
            AelucanthThoraxS => "Tułów Aelucantha S",
            TigrexHelm => "Hełm z Tigrexa",
            UroktorCoilS => "Pas z Uroktora S",
            NargacugaBracesS => "Zaręk. z Nargacuga S",
            AnjanathVambraces => "Zaręk. z Anjanatha",
            ShellStuddedVestS => "Muszlowa kamizelka S",
            AnjanathHelm => "Hełm z Anjanatha",
            SinisterHelm => "Złowrogi hełm",
            ChainmailBelt => "Pas kolczy",
            AlmudronVambracesS => "Zaręk. z Almudrona S",
            BishatenVambracesS => "Zaręk. z Bishatena S",
            AelucanthBrachiaS => "Ramiona Aelucantha S",
            ChannelersRobe => "Szata pośrednika",
            KhezuMail => "Zbroja z Khezu",
            EdelVizor => "Przyłbica Edel",
            ZinogreMail => "Zbroja z Zinogra",
            EdelVizorS => "Przyłbica Edel S",
            DrothGreaves => "Nagolenice z Drotha",
            TetranadonHelm => "Hełm z Tetranadona",
            DrothMail => "Zbroja z Drotha",
            VolvidonMailS => "Zbroja z Volvidona S",
            KamuraBracesS => "Zarękawia Kamury S",
            DamascusGreaves => "Nagol. damasceńskie",
            DiablosMail => "Zbroja z Diablosa",
            BariothMailS => "Zbroja z Bariotha S",
            RhenoplosMail => "Zbroja z Rhenoplosa",
            BishatenVambraces => "Zaręk. z Bishatena",
            AlmudronGreaves => "Nagol. z Almudrona",
            GossHaragBracesS => "Zaręk. z Goss Har. S",
            ChaosPlate => "Płyta Chaosu",
            BasariosGreaves => "Nagol. z Basariosa",
            DoberMail => "Zbroja z Dobera",
            AnjanathHelmS => "Hełm z Anjanatha S",
            DoberGreaves => "Nagolenice z Dobera",
            AlloyGreavesS => "Nagol. ze stopu S",
            NargacugaMailS => "Zbroja z Nargacuga S",
            DrothCoil => "Pas z Drotha",
            TigrexGreaves => "Nagolenice z Tigrexa",
            WroggiVambraces => "Zaręk. z Wroggiego",
            RhenoplosCoilS => "Pas z Rhenoplosa S",
            BariothHelmS => "Hełm z Bariotha S",
            WroggiMail => "Zbroja z Wroggiego",
            SkaldaBrachia => "Ramiona Skaldy",
            RathalosCoil => "Pas z Rathalosa",
            GoldenHeaddress => "Złote przybranie",
            TetranadonGreavesS => "Nag. z Tetranadona S",
            RaknaGreaves => "Nagolenice z Rakny",
            DamascusCoil => "Pas damasceński",
            UtsushiTassetsHS => "Taszki Utsush. (H) S",
            IngotCoil => "Pas ze sztabek",
            BnahabraSuit => "Strój z Bnahabry",
            KadachiGreaves => "Nagolen. z Kadachi",
            BasariosVambraces => "Zaręk. z Basariosa",
            MediumsPrayer => "Modlitwa medium",
            BarrothCoilS => "Pas z Barrotha S",
            RemobraSuitS => "Strój z Remobry S",
            NarwasFauld => "Fartuch z Narwy",
            BarrothGreaves => "Nagol. z Barrotha",
            IzuchiHelm => "Hełm z Izuchiego",
            TigrexCoilS => "Pas z Tigrexa S",
            MizutsuneCoilS => "Pas z Mizutsune S",
            LudrothBraces => "Zarękawia z Ludrotha",
            LeatherGloves => "Skórzane rękawice",
            SpioThorax => "Tułów Spio",
            SpioElytraS => "Okrywy Spio S",
            UtsushiChestH => "Tułów Utsushiego (H)",
            AlmudronHelmS => "Hełm z Almudrona S",
            SpioBrachiaS => "Ramiona Spio S",
            SpioElytra => "Okrywy Spio",
            IzuchiCoilS => "Pas z Izuchiego S",
            BasariosMail => "Zbroja z Basariosa",
            LudrothHelm => "Hełm z Ludrotha",
            BarrothHelm => "Hełm z Barrotha",
            AlloyMail => "Zbroja ze stopu",
            UtsushiBracesH => "Zarękaw. Utsush. (H)",
            KamuraiHat => "Kapelusz Kamuraj",
            LagombiCoilS => "Pas z Lagombiego S",
            BrigadeCoil => "Pas Brygady",
            SinisterHelmS => "Złowrogi hełm S",
            DeathStenchBowels => "Jelita rozkładu",
            DiablosHelmS => "Hełm z Diablosa S",
            RhenoplosHelmS => "Hełm z Rhenoplosa S",
            LeatherGlovesS => "Skórzane rękawice S",
            MosgharlRootsS => "Korzenie Mosgharl S",
            RemobraHeadgearS => "Hełm z Remobry S",
            BrigadeBoots => "Buty Brygady",
            MediumsHairtie => "Opaska medium",
            KhezuHelmS => "Hełm z Khezu S",
            JellyCoilS => "Pas Jelly S",
            BasariosHelm => "Hełm z Basariosa",
            KuluYaKuCoil => "Pas z Kulu-Ya-Ku",
            JellyBoots => "Buty Jelly",
            EdelRibplateS => "Napierśnik Edel S",
            MightyBowFeather => "Pióro potężn. łuku",
            IzuchiHelmS => "Hełm z Izuchiego S",
            MakluvaCoverS => "Osłona Makluva S",
            VaikHelm => "Hełm Vaik",
            EdelFoliaS => "Listowie Edel S",
            SkullVisage => "Oblicze czaszki",
            KadachiHelm => "Hełm z Kadachi",
            JaggiShinguards => "Nagol. z Jaggiego",
            GossHaragCoilS => "Pas z Goss Haraga S",
            MizuhaSleeves => "Rękawy Mizuha",
            BariothVambraces => "Zarękawia z Bariotha",
            RemobraBeltS => "Pas z Remobry S",
            MizutsuneHelmS => "Hełm z Mizutsune S",
            SomnacanthMailS => "Zbr. z Somnacan. S",
            SpioVertex => "Czub Spio",
            WroggiCoil => "Pas z Wroggiego",
            DrothMailS => "Zbroja z Drotha S",
            BullfangoMask => "Maska Bullfango",
            ZinogreCoil => "Pas z Zinogra",
            LudrothMailS => "Zbroja z Ludrotha S",
            KuluYaKuHelmS => "Hełm z Kulu-Ya-Ku S",
            DiablosHelm => "Hełm z Diablosa",
            LagombiVambracesS => "Zar. z Lagombiego S",
            SpioBrachia => "Ramiona Spio",
            KamuraGarb => "Strój Kamury",
            ShelledSandalsS => "Muszlowe sandały S",
            KhezuGreaves => "Nagolenice z Khezu",
            FootofNarwa => "Stopa Narwy",
            TigrexMailS => "Zbroja z Tigrexa S",
            UtsushiChestHS => "Tułów Utsush. (H) S",
            BazelgeuseMail => "Zbroja z Bazelgeza",
            LudrothCoilS => "Pas z Ludrotha S",
            TigrexGreavesS => "Nagol. z Tigrexa S",
            VaikCoil => "Pas Vaik",
            KuluYaKuMail => "Zbroja z Kulu-Ya-Ku",
            WroggiGreavesS => "Nagol. z Wroggiego S",
            LudrothGreavesS => "Nagol. z Ludrotha S",
            BoneCoilS => "Kościany pas S",
            JyuratodusVambraces => "Zaręk. z Jyuratodusa",
            NargacugaHelmS => "Hełm z Nargacuga S",
            BrigadeLobosS => "Kapelusz Brygady S",
            AlloyVambraces => "Zarękawia ze stopu",
            AknosomCoil => "Pas z Aknosoma",
            ChainmailGlovesS => "Rękawice kolcze S",
            AknosomMail => "Zbroja z Aknosoma",
            VolvidonMail => "Zbroja z Volvidona",
            SkaldaCrura => "Nogi Skaldy",
            ZinogreCoilS => "Pas z Zinogra S",
            MosgharlFolia => "Listowie Mosgharl",
            SwallowShirt => "Jaskółcza koszula",
            EdelCreeperS => "Pnącze Edel S",
            NargacugaHelm => "Hełm z Nargacuga",
            IbushisPauldrons => "Naram. z Ibushiego",
            BrigadeSuit => "Strój Brygady",
            KaiserMail => "Zbroja kajzera",
            SkaldaThoraxS => "Tułów Skaldy S",
            TetranadonCoilS => "Pas z Tetranadona S",
            LagombiVambraces => "Zaręk. z Lagombiego",
            SomnacanthCoilS => "Pas z Somnacantha S",
            TetranadonBraces => "Zaręk. z Tetranadona",
            AnjanathVambracesS => "Zaręk. z Anjanatha S",
            GargwaMaskS => "Maska z Gargwy S",
            GossHaragCoil => "Pas z Goss Haraga",
            SinisterSealMask => "Maska złowr. pieczęci",
            AlmudronGreavesS => "Nagol. z Almudrona S",
            UtsushiMaskHS => "Maska Utsush. (H) S",
            BoneVambraces => "Kościane zarękawia",
            IzuchiGreaves => "Nagol. z Izuchiego",
            TheaterWig => "Teatralna peruka",
            SwallowBoots => "Jaskółcze buty",
            ArzurosHelm => "Hełm z Arzurosa",
            ChainmailBeltS => "Pas kolczy S",
            BaggiVambraces => "Zarękawia z Baggiego",
            IbushisHelm => "Hełm Ibushiego",
            KuluYaKuGreavesS => "Nag. z Kulu-Ya-Ku S",
            DoberHelm => "Hełm z Dobera",
            RemobraGlovesS => "Rękawice z Remobry S",
            AelucanthElytraS => "Okrywy Aelucantha S",
            ChannelersHakamaS => "Hakama pośrednika S",
            BariothCoilS => "Pas z Bariotha S",
            JellyCoil => "Pas Jelly",
            MelahoaJacket => "Kubrak Melahoa",
            BnahabraHatS => "Kap. z Bnahabry S",
            KhezuVambraces => "Zarękawia z Khezu",
            AlloyVambracesS => "Zarękawia ze stopu S",
            IngotVambracesS => "Zaręk. ze sztabek S",
            SkaldaBrachiaS => "Ramiona Skaldy S",
            DeathStenchBrainS => "Mózg rozkładu S",
            ZinogreGreavesS => "Nagol. z Zinogra S",
            DiablosVambraces => "Zarękawia z Diablosa",
            SStuddedSashS => "Gwiezdna szarfa S",
            MosgharlRibplate => "Napierś. Mosgharl",
            MizuhaCap => "Czapka Mizuha",
            KuluYaKuCoilS => "Pas z Kulu-Ya-Ku S",
            PukeiPukeiCoilS => "Pas z Pukei-Pukei S",
            BazelgeuseBraces => "Zaręk. z Bazelgeza",
            ChainmailPants => "Spodnie kolcze",
            BrigadeVambracesS => "Zarękawia Brygady S",
            BrigadeBootsS => "Buty Brygady S",
            AknosomHelmS => "Hełm z Aknosoma S",
            BnahabraGlovesS => "Rękawice z Bnahab. S",
            JaggiShinguardsS => "Nagol. z Jaggiego S",
            SomnacanthMail => "Zbr. z Somnacantha",
            ZinogreGreaves => "Nagol. z Zinogra",
            ChainmailHeadgearS => "Hełm kolczy S",
            BrigadeVambraces => "Zarękawia Brygady",
            BazelgeuseCoil => "Pas z Bazelgeza",
            SkaldaVertexS => "Czub Skaldy S",
            VolvidonCoilS => "Pas z Volvidona S",
            DeathStenchHeel => "Pięta rozkładu",
            SomnacanthGreaves => "Nag. z Somnacantha",
            RhenoplosCoil => "Pas z Rhenoplosa",
            BnahabraSuitS => "Strój z Bnahabry S",
            MosgharlFoliaS => "Listowie Mosgharl S",
            UtsushiMaskV => "Maska Utsushiego (V)",
            AlmudronMail => "Zbroja z Almudrona",
            AnjanathMail => "Zbroja z Anjanatha",
            KamuraObiS => "Obi Kamury S",
            LeatherHeadgear => "Skórzany kaptur",
            ChannelersObi => "Amulet pośrednika",
            MakluvaCover => "Osłona Makluva",
            RhopessaThorax => "Tułów Rhopessy",
            IzuchiBraces => "Zaręk. z Izuchiego",
            TobiKadachiMail => "Zbr. z Tobi-Kadachi",
            MizutsuneHelm => "Hełm z Mizutsune",
            BaggiVambracesS => "Zaręk. z Baggiego S",
            BariothMail => "Zbroja z Bariotha",
            SinisterTassetsS => "Złowr. nabiodrki S",
            NarwaBreastplate => "Napierśnik z Narwy",
            KhezuCoilS => "Pas z Khezu S",
            JyuratodusHelm => "Hełm z Jyuratodusa",
            ChannelersHairtieS => "Opaska pośrednika S",
            UtsushiTassetsH => "Taszki Utsush. (H)",
            KushalaGrip => "Chwyt Kushali",
            JaggiMask => "Maska z Jaggiego",
            DrothCoilS => "Pas z Drotha S",
            MosgharlCreeper => "Pnącze Mosgharl",
            IngotGreavesS => "Nagol. ze sztabek S",
            AknosomBracesS => "Zaręk. z Aknosoma S",
            GossHaragMail => "Zbr. z Goss Haraga",
            SlagtothHoodS => "Kapt. ze Slagtotha S",
            HuntersMail => "Zbroja łowcy",
            UroktorCoil => "Pas z Uroktora",
            GossHaragGreaves => "Nag. z Goss Haraga",
            ChaosArchplate => "Arcypłyta Chaosu",
            IzuchiMailS => "Zbr. z Izuchiego S",
            DeathStenchGripS => "Chwyt rozkładu S",
            SomnacanthBraces => "Zaręk. z Somnacantha",
            MakluvaHood => "Kaptur Makluva",
            VaikGreavesS => "Nagolenice Vaik S",
            UtsushiBracesV => "Zarękaw. Utsush. (V)",
            BoneGreaves => "Kościane nagolenice",
            RhopessaElytraS => "Okrywy Rhopessy S",
            CunningSpecs => "Szkła przebiegłości",
            LagombiGreaves => "Nag. z Lagombiego",
            UtsushiChestV => "Tułów Utsushiego (V)",
            RathianBraces => "Zarękawia z Rathiany",
            DeathStenchBrain => "Mózg rozkładu",
            RemobraFeet => "Stopy Remobry",
            DeathStenchMuscle => "Mięsień rozkładu",
            HuntersCoil => "Pas łowcy",
            LudrothMail => "Zbroja z Ludrotha",
            UtsushiMaskH => "Maska Utsushiego (H)",
            KamuraiLeggings => "Nogawice Kamuraj",
            ArzurosVambracesS => "Zaręk. z Arzurosa S",
            RhopessaBrachiaS => "Ramiona Rhopessy S",
            DiablosMailS => "Zbroja z Diablosa S",
            ValstraxCoil => "Pas z Valstraxa",
            SinisterGauntlets => "Złowrogie rękawice",
            SinisterGarbS => "Złowrogi strój S",
            EdelFolia => "Listowie Edel",
            DiablosGreavesS => "Nagol. z Diablosa S",
            BoneCoil => "Kościany pas",
            KhezuCoil => "Pas z Khezu",
            JellyBootsS => "Buty Jelly S",
            AlloyGreaves => "Nagolenice ze stopu",
            MelahoaFoliaS => "Listowie Melahoa S",
            MelahoaHatS => "Kapelusz Melahoa S",
            PukeiPukeiMail => "Zbroja z Pukei-Pukei",
            ChainmailVest => "Kolczuga",
            RhenoplosHelm => "Hełm z Rhenoplosa",
            RathianGreaves => "Nagol. z Rathiany",
            MosgharlRoots => "Korzenie Mosgharl",
            AknosomHelm => "Hełm z Aknosoma",
            ChainmailPantsS => "Spodnie kolcze S",
            TetranadonHelmS => "Hełm z Tetranadona S",
            ZinogreHelm => "Hełm z Zinogra",
            BarrothMailS => "Zbroja z Barrotha S",
            ArzurosGreaves => "Nagol. z Arzurosa",
            BoneMail => "Kościana zbroja",
            LagombiMailS => "Zbr. z Lagombiego S",
            AnjanathGreaves => "Nagol. z Anjanatha",
            UtsushiChestVS => "Tułów Utsush. (V) S",
            RathianCoilS => "Pas z Rathiany S",
            ShelledSandals => "Muszlowe sandały",
            TobiKadachiMailS => "Zb. z Tobi-Kadachi S",
            PukeiPukeiBracesS => "Zar. z Pukei-Pukei S",
            BaggiCoilS => "Pas z Baggiego S",
            SinisterGreaves => "Złowrogie nagol.",
            BishatenCoil => "Pas z Bishatena",
            NarwasHelm => "Hełm Narwy",
            FeatherofMastery => "Pióro Mistrzostwa",
            BariothCoil => "Pas z Bariotha",
            TigrexMail => "Zbroja z Tigrexa",
            TobiKadachiCoil => "Pas z Tobi-Kadachi",
            IngotCoilS => "Pas ze sztabek S",
            IzuchiBracesS => "Zaręk. z Izuchiego S",
            ChannelersHakama => "Hakama pośrednika",
            AlloyHelmS => "Hełm ze stopu S",
            VolvidonVambracesS => "Zaręk. z Volvidona S",
            LeatherVest => "Kamizela skórzana",
            BoneVambracesS => "Kościane zarękawia S",
            BaggiGreaves => "Nagol. z Baggiego",
            SinisterSealGarb => "Szata złowr. pieczęci",
            MosgharlVizorS => "Przyłbica Mosgharl S",
            ShellStuddedVest => "Muszlowa kamizelka",
            SomnacanthCoil => "Pas z Somnacantha",
            RhenoplosGreavesS => "Nag. z Rhenoplosa S",
            RathianBracesS => "Zaręk. z Rathiany S",
            BasariosGreavesS => "Nagol. z Basariosa S",
            KamuraHeadScarfS => "Chustka Kamury S",
            BaggiCoil => "Pas z Baggiego",
            KuluYaKuGreaves => "Nagol. z Kulu-Ya-Ku",
            MizutsuneCoil => "Pas z Mizutsune",
            VaikBracesS => "Zarękawia Vaik S",
            ChannelersHopeS => "Nadzieja pośredn. S",
            AnjanathGreavesS => "Nagol. z Anjanatha S",
            HuntersGreavesS => "Nagolenice łowcy S",
            RhenoplosBraces => "Zaręk. z Rhenoplosa",
            WroggiHelm => "Hełm z Wroggiego",
            AknosomMailS => "Zbr. z Aknosoma S",
            DiablosVambracesS => "Zaręk. z Diablosa S",
            RhopessaVertexS => "Czub Rhopessy S",
            EdelRoots => "Korzenie Edel",
            BaggiMail => "Zbroja z Baggiego",
            MediumsHakama => "Hakama medium",
            DamascusMail => "Zbroja damasceńska",
            FoxMask => "Maska lisa",
            HuntersHelmS => "Hełm łowcy S",
            GoldenKote => "Złote kote",
            PukeiPukeiCoil => "Pas z Pukei-Pukei",
            VaikGreaves => "Nagolenice Vaik",
            WroggiGreaves => "Nagol. z Wroggiego",
            RhopessaElytra => "Okrywy Rhopessy",
            RhopessaCrura => "Nogi Rhopessy",
            MakluvaHoodS => "Kaptur Makluva S",
            AknosomCoilS => "Pas z Aknosoma S",
            NarwasPauldrons => "Naramienniki z Narwy",
            SlagtothHood => "Kaptur ze Slagtotha",
            BaggiGreavesS => "Nagol. z Baggiego S",
            BarrothCoil => "Pas z Barrotha",
            MizuhaGaiters => "Kamasze Mizuha",
            ValstraxMail => "Zbroja z Valstraxa",
            JyuratodusCoil => "Pas z Jyuratodusa",
            EdelRibplate => "Napierśnik Edel",
            ZinogreBraces => "Zarękawia z Zinogra",
            MelahoaBranchS => "Gałąź Melahoa S",
            BazelgeuseHelm => "Hełm z Bazelgeza",
            EdelRootsS => "Korzenie Edel S",
            MelahoaHat => "Kapelusz Melahoa",
            PukeiGreaves => "Nagolenice z Pukei",
            MizutsuneMailS => "Zbroja z Mizutsune S",
            GoldenHaori => "Złote haori",
            LagombiMail => "Zbroja z Lagombiego",
            MediumsHakamaS => "Hakama medium S",
            BasariosCoilS => "Pas z Basariosa S",
        }
    }
}

impl Polish for super::UiSymbole {
    fn to_polish(&self) -> &'static str {
        super::en::English::to_english(self)
    }
}
