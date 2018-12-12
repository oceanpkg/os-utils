//! Utilities specific to Ubuntu Linux.

use version::Version;

use self::OsRelease::*;

/// The release name of a known Ubuntu version.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum OsRelease {
    /// Feisty Fawn (Ubuntu 7.04, Linux 2.6.20).
    FeistyFawn,
    /// Gutsy Gibbon (Ubuntu 7.10, Linux 2.6.22).
    GutsyGibbon,
    /// Hardy Heron (Ubuntu 8.04, Linux 2.6.24).
    HardyHeron,
    /// Intrepid Ibex (Ubuntu 8.10, Linux 2.6.27).
    IntrepidIbex,
    /// Jaunty Jackalope (Ubuntu 9.04, Linux 2.6.28).
    JauntyJackalope,
    /// Karmic Koala (Ubuntu 9.10, Linux 2.6.31).
    KarmicKoala,
    /// Lucid Lynx (Ubuntu 10.04, Linux 2.6.32).
    LucidLynx,
    /// Maverick Meerkat (Ubuntu 10.10, Linux 2.6.35).
    MaverickMeerkat,
    /// Natty Narwhal (Ubuntu 11.04, Linux 2.6.38).
    NattyNarwhal,
    /// Oneiric Ocelot (Ubuntu 11.10, Linux 3.0).
    OneiricOcelot,
    /// Precise Pangolin (Ubuntu 12.04, Linux 3.2+).
    PrecisePangolin,
    /// Quantal Quetzal (Ubuntu 12.10, Linux 3.5).
    QuantalQuetzal,
    /// Raring Ringtail (Ubuntu 13.04, Linux 3.8).
    RaringRingtail,
    /// Saucy Salamander (Ubuntu 13.10, Linux 3.11).
    SaucySalamander,
    /// Trusty Tahr (Ubuntu 14.04, Linux 3.13).
    TrustyTahr,
    /// Utopic Unicorn (Ubuntu 14.10, Linux 3.16).
    UtopicUnicorn,
    /// Vivid Vervet (Ubuntu 15.04, Linux 3.19).
    VividVervet,
    /// Wily Werewolf (Ubuntu 15.10, Linux 4.2).
    WilyWerewolf,
    /// Xenial Xerus (Ubuntu 16.04, Linux 4.4).
    XenialXerus,
    /// Yakkety Yak (Ubuntu 16.10, Linux 4.8).
    YakketyYak,
    /// Zesty Zapus (Ubuntu 17.04, Linux 4.10).
    ZestyZapus,
    /// Artful Aardvark (Ubuntu 17.10, Linux 4.13).
    ArtfulAardvark,
    /// Bionic Beaver (Ubuntu 18.04, Linux 4.15).
    BionicBeaver,
    /// Cosmic Cuttlefish (Ubuntu 18.10, Linux 4.18).
    CosmicCuttlefish,
    // We assume this value will never be used, so `unreachable_unchecked()` is
    // fine to use
    #[doc(hidden)]
    _NonExhaustive,
}

impl<'a> From<OsRelease> for &'a str {
    fn from(release: OsRelease) -> Self {
        match release {
            FeistyFawn       => "Feisty Fawn",
            GutsyGibbon      => "Gutsy Gibbon",
            HardyHeron       => "Hardy Heron",
            IntrepidIbex     => "Intrepid Ibex",
            JauntyJackalope  => "Jaunty Jackalope",
            KarmicKoala      => "Karmic Koala",
            LucidLynx        => "Lucid Lynx",
            MaverickMeerkat  => "Maverick Meerkat",
            NattyNarwhal     => "Natty Narwhal",
            OneiricOcelot    => "Oneiric Ocelot",
            PrecisePangolin  => "Precise Pangolin",
            QuantalQuetzal   => "Quantal Quetzal",
            RaringRingtail   => "Raring Ringtail",
            SaucySalamander  => "Saucy Salamander",
            TrustyTahr       => "Trusty Tahr",
            UtopicUnicorn    => "Utopic Unicorn",
            VividVervet      => "Vivid Vervet",
            WilyWerewolf     => "Wily Werewolf",
            XenialXerus      => "Xenial Xerus",
            YakketyYak       => "Yakkety Yak",
            ZestyZapus       => "Zesty Zapus",
            ArtfulAardvark   => "Artful Aardvark",
            BionicBeaver     => "Bionic Beaver",
            CosmicCuttlefish => "Cosmic Cuttlefish",
            _NonExhaustive   => unsafe { std::hint::unreachable_unchecked() },
        }
    }
}

impl OsRelease {
    /// The minimum supported OS release.
    pub const MIN: OsRelease = FeistyFawn;

    /// The most recent OS release.
    pub const LATEST: OsRelease = CosmicCuttlefish;

    // This might be a large table; monomorphize here
    fn _new(Version { major, minor, .. }: Version) -> Option<Self> {
        match (major, minor) {
            (07, 04) => Some(FeistyFawn),
            (07, 10) => Some(GutsyGibbon),
            (08, 04) => Some(HardyHeron),
            (08, 10) => Some(IntrepidIbex),
            (09, 04) => Some(JauntyJackalope),
            (09, 10) => Some(KarmicKoala),
            (10, 04) => Some(LucidLynx),
            (10, 10) => Some(MaverickMeerkat),
            (11, 04) => Some(NattyNarwhal),
            (11, 10) => Some(OneiricOcelot),
            (12, 04) => Some(PrecisePangolin),
            (12, 10) => Some(QuantalQuetzal),
            (13, 04) => Some(RaringRingtail),
            (13, 10) => Some(SaucySalamander),
            (14, 04) => Some(TrustyTahr),
            (14, 10) => Some(UtopicUnicorn),
            (15, 04) => Some(VividVervet),
            (15, 10) => Some(WilyWerewolf),
            (16, 04) => Some(XenialXerus),
            (16, 10) => Some(YakketyYak),
            (17, 04) => Some(ZestyZapus),
            (17, 10) => Some(ArtfulAardvark),
            (18, 04) => Some(BionicBeaver),
            (18, 10) => Some(CosmicCuttlefish),
            _ => None,
        }
    }

    /// Returns the corresponding release for the Ubuntu version number.
    pub fn new<V: Into<Version>>(version: V) -> Option<Self> {
        Self::_new(version.into())
    }

    /// Returns the minimum kernel version for the release.
    pub fn min_kernel_version(self) -> Version {
        match self {
            FeistyFawn       => (2, 06, 20),
            GutsyGibbon      => (2, 06, 22),
            HardyHeron       => (2, 06, 24),
            IntrepidIbex     => (2, 06, 27),
            JauntyJackalope  => (2, 06, 28),
            KarmicKoala      => (2, 06, 31),
            LucidLynx        => (2, 06, 32),
            MaverickMeerkat  => (2, 06, 35),
            NattyNarwhal     => (2, 06, 38),
            OneiricOcelot    => (3, 00, 00),
            PrecisePangolin  => (3, 02, 00),
            QuantalQuetzal   => (3, 05, 00),
            RaringRingtail   => (3, 08, 00),
            SaucySalamander  => (3, 11, 00),
            TrustyTahr       => (3, 13, 00),
            UtopicUnicorn    => (3, 16, 00),
            VividVervet      => (3, 19, 00),
            WilyWerewolf     => (4, 02, 00),
            XenialXerus      => (4, 04, 00),
            YakketyYak       => (4, 08, 00),
            ZestyZapus       => (4, 10, 00),
            ArtfulAardvark   => (4, 13, 00),
            BionicBeaver     => (4, 15, 00),
            CosmicCuttlefish => (4, 18, 00),
            _NonExhaustive   => unsafe { std::hint::unreachable_unchecked() },
        }.into()
    }
}

impl From<OsRelease> for Version {
    fn from(release: OsRelease) -> Version {
        match release {
            FeistyFawn       => (07, 04),
            GutsyGibbon      => (07, 10),
            HardyHeron       => (08, 04),
            IntrepidIbex     => (08, 10),
            JauntyJackalope  => (09, 04),
            KarmicKoala      => (09, 10),
            LucidLynx        => (10, 04),
            MaverickMeerkat  => (10, 10),
            NattyNarwhal     => (11, 04),
            OneiricOcelot    => (11, 10),
            PrecisePangolin  => (12, 04),
            QuantalQuetzal   => (12, 10),
            RaringRingtail   => (13, 04),
            SaucySalamander  => (13, 10),
            TrustyTahr       => (14, 04),
            UtopicUnicorn    => (14, 10),
            VividVervet      => (15, 04),
            WilyWerewolf     => (15, 10),
            XenialXerus      => (16, 04),
            YakketyYak       => (16, 10),
            ZestyZapus       => (17, 04),
            ArtfulAardvark   => (17, 10),
            BionicBeaver     => (18, 04),
            CosmicCuttlefish => (18, 10),
            _NonExhaustive   => unsafe { std::hint::unreachable_unchecked() },
        }.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cmp() {
        assert!(OsRelease::LATEST > OsRelease::MIN);
    }
}
