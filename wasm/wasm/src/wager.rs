//! Pascal's wager over every tradition at once.
//!
//! The classical wager compares two columns, believe or don't, against two
//! rows, God or no God, and finds an infinity in one cell. The reply that has
//! been made since Diderot is that other religions promise infinities of their own,
//! in different cells. This module writes those cells out: for each tradition
//! taken as true (the row) and each tradition followed (the column), what that
//! tradition says becomes of such a person after death.
//!
//! An outcome is `eternal · ∞ + finite`. `eternal` is the coefficient on
//! infinity: `+1` for paradise without end, `-1` for torment without end, a
//! fraction where the tradition itself leaves the matter to chance or gives an
//! eternity that is neither bliss nor pain, `0` where nothing lasts forever.
//! `finite` is everything that ends, measured in ordinary human lifetimes: a
//! purgatory, a hell that empties, a paradise until Ragnarök, a run of rebirths.
//! Expected utility over a prior on the rows is then a number of the same shape,
//! and two of them compare lexicographically: the infinite part first, and the
//! finite part only when the infinite parts tie. That is the only order under
//! which an expected value involving infinities means anything, and it is what
//! makes the wager computable at all: `∞ − ∞` never has to be evaluated.
//!
//! Every follower is assumed to be the same decent person; only creed, rite and
//! diet differ between the columns. A tradition that judges by conduct alone
//! therefore gives every column the same verdict, and one that judges by creed
//! does not. The verdicts are one reader's summary of what each tradition
//! teaches, and the note on every cell says which teaching it is based on.

use wasm_bindgen::prelude::*;

/// A family of traditions, for verdicts that treat related traditions alike.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Family {
    Philosophical,
    Christian,
    Islamic,
    Jewish,
    Hindu,
    Buddhist,
    Jain,
    Sikh,
    EastAsian,
    Iranian,
    Gnostic,
    Ancient,
    African,
    NewReligious,
}

impl Family {
    fn label(self) -> &'static str {
        match self {
            Family::Philosophical => "philosophies",
            Family::Christian => "Christianity",
            Family::Islamic => "Islam",
            Family::Jewish => "Judaism",
            Family::Hindu => "Hinduism",
            Family::Buddhist => "Buddhism",
            Family::Jain => "Jainism",
            Family::Sikh => "Sikhism",
            Family::EastAsian => "East Asian",
            Family::Iranian => "Iranian and Levantine",
            Family::Gnostic => "gnostic and dualist",
            Family::Ancient => "ancient",
            Family::African => "African",
            Family::NewReligious => "new religious movements",
        }
    }
}

/// What a tradition says about gods, as far as another tradition's verdict
/// cares.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Theism {
    Monotheist,
    Polytheist,
    Nontheist,
    Atheist,
    Pantheist,
    Deist,
    Dualist,
}

/// One tradition, with the traits other traditions' verdicts look at.
#[derive(Debug)]
pub struct Religion {
    pub id: &'static str,
    pub name: &'static str,
    pub family: Family,
    pub theism: Theism,
    /// Worships images or many gods, by the Abrahamic reckoning of idolatry.
    pub idols: bool,
    /// Disposes of the body by fire, which matters where the body is needed.
    pub cremates: bool,
    /// A follower does not eat meat.
    pub vegetarian: bool,
    /// Holds a death in battle to be a good death.
    pub martial: bool,
    /// Kills animals for the gods.
    pub sacrifice: bool,
    /// Net cost of a lifetime of practice, in lifetimes: tithes, fasts,
    /// prayers, dietary law, fees, and the risks the practice asks you to
    /// take, less whatever the practice gives back in this life.
    pub cost: f64,
    /// Share of the world's population, in percent, for the prior that
    /// weights each tradition by how many people follow it.
    pub adherents: f64,
    /// What the tradition itself says happens after death, in one sentence.
    pub summary: &'static str,
}

impl Religion {
    fn is(&self, id: &str) -> bool {
        self.id == id
    }

    fn any(&self, ids: &[&str]) -> bool {
        ids.contains(&self.id)
    }

    fn christian(&self) -> bool {
        self.family == Family::Christian
    }

    /// Christian by the creeds of Nicaea and Chalcedon, which is what the
    /// mutual recognition of baptism between the older churches is based on.
    fn creedal_christian(&self) -> bool {
        self.christian() && !self.any(&["lds", "jw"])
    }

    fn muslim(&self) -> bool {
        self.family == Family::Islamic
    }

    fn jewish(&self) -> bool {
        self.family == Family::Jewish
    }

    fn abrahamic(&self) -> bool {
        self.christian() || self.muslim() || self.jewish()
    }

    fn buddhist(&self) -> bool {
        self.family == Family::Buddhist
    }

    fn dharmic(&self) -> bool {
        matches!(
            self.family,
            Family::Hindu | Family::Buddhist | Family::Jain | Family::Sikh
        )
    }

    fn godless(&self) -> bool {
        matches!(self.theism, Theism::Atheist | Theism::Pantheist)
    }
}

/// Where a row's verdicts come from, beyond the primary texts the notes cite.
#[derive(Debug)]
pub struct Source {
    pub label: &'static str,
    pub url: &'static str,
}

macro_rules! source {
    ($label:literal, $url:literal) => {
        Source {
            label: $label,
            url: $url,
        }
    };
}

/// The reading behind each row, one entry per tradition. Naturalism has none:
/// its verdict is that nothing happens, which no source is needed for.
pub const SOURCES: &[(&str, &[Source])] = &[
    ("atheism", &[]),
    (
        "deism",
        &[
            source!(
                "Paine, The Age of Reason",
                "https://www.ushistory.org/paine/reason/reason37.htm"
            ),
            source!(
                "The Age of Reason (Wikipedia)",
                "https://en.wikipedia.org/wiki/The_Age_of_Reason"
            ),
        ],
    ),
    (
        "pantheism",
        &[
            source!(
                "Eternity of Mind, The Cambridge Spinoza Lexicon",
                "https://www.cambridge.org/core/books/abs/cambridge-spinoza-lexicon/eternity-of-mind/E69C6FA558EE8AD2C6E91459495C27D6"
            ),
            source!(
                "Nadler, Eternity and Immortality in Spinoza's Ethics",
                "https://www.researchgate.net/publication/228028265_Eternity_and_Immortality_in_Spinoza's_Ethics"
            ),
        ],
    ),
    (
        "catholic",
        &[
            source!(
                "Lumen Gentium 14-16",
                "https://www.vatican.va/archive/hist_councils/ii_vatican_council/documents/vat-ii_const_19641121_lumen-gentium_en.html"
            ),
            source!(
                "Bullivant, Vatican II and Inculpable Ignorance",
                "https://theologicalstudies.net/wp-content/uploads/2022/08/004056391107200104.pdf"
            ),
        ],
    ),
    (
        "orthodox",
        &[
            source!(
                "Kalomiros, The River of Fire",
                "https://glory2godforallthings.com/the-river-of-fire-kalomiros/"
            ),
            source!(
                "The Orthodox view of hell as God's love",
                "https://orthocath.wordpress.com/2010/09/16/hell-and-gods-love-an-orthodox-view/"
            ),
        ],
    ),
    (
        "mainline",
        &[source!(
            "Rahner's anonymous Christians",
            "https://en.wikipedia.org/wiki/Anonymous_Christian"
        )],
    ),
    (
        "evangelical",
        &[
            source!(
                "Evangelicalism (Wikipedia)",
                "https://en.wikipedia.org/wiki/Evangelicalism"
            ),
            source!(
                "Christian views on hell",
                "https://en.wikipedia.org/wiki/Christian_views_on_Hell"
            ),
        ],
    ),
    (
        "calvinist",
        &[
            source!(
                "Westminster Confession, chapter 3",
                "https://www.christianstudylibrary.org/article/westminster-confession-faith-chapter-3-predestined-unto-life-christ"
            ),
            source!(
                "Predestination in Calvinism",
                "https://en.wikipedia.org/wiki/Predestination_in_Calvinism"
            ),
        ],
    ),
    (
        "lds",
        &[
            source!(
                "Kingdoms of Glory and Perdition (Church manual)",
                "https://www.churchofjesuschrist.org/study/manual/doctrines-of-the-gospel-student-manual/33-kingdoms-of-glory-and-perdition?lang=eng"
            ),
            source!(
                "Son of perdition (Mormonism)",
                "https://en.wikipedia.org/wiki/Son_of_perdition_(Mormonism)"
            ),
        ],
    ),
    (
        "jw",
        &[
            source!(
                "Jehovah's Witnesses and salvation",
                "https://en.wikipedia.org/wiki/Jehovah%27s_Witnesses_and_salvation"
            ),
            source!(
                "Eschatology of Jehovah's Witnesses",
                "https://en.wikipedia.org/wiki/Eschatology_of_Jehovah%27s_Witnesses"
            ),
        ],
    ),
    (
        "adventist",
        &[
            source!(
                "Conditionalism, a cornerstone of Adventist doctrine",
                "https://www.ministrymagazine.org/archive/1986/08/conditionalism-a-cornerstone-of-adventist-doctrine"
            ),
            source!(
                "Adventist beliefs on death and resurrection",
                "https://www.askanadventistfriend.com/adventist-beliefs/death-and-resurrection/"
            ),
        ],
    ),
    (
        "universalist",
        &[
            source!(
                "Apocatastasis (Catholic Encyclopedia)",
                "https://www.newadvent.org/cathen/01599a.htm"
            ),
            source!(
                "Gregory of Nyssa on 1 Corinthians 15:28",
                "https://www.mercyuponall.org/2018/11/13/gregory-of-nyssa-a-treatise-on-first-corinthians-1528-in-illud/"
            ),
        ],
    ),
    (
        "sunni",
        &[
            source!(
                "Jahannam (Wikipedia)",
                "https://en.wikipedia.org/wiki/Jahannam"
            ),
            source!(
                "An atom's weight of faith (IslamQA)",
                "https://islamqa.info/en/answers/170526"
            ),
            source!(
                "Khalil, Temporary Hellfire and the Making of Sunni Orthodoxy",
                "https://www.academia.edu/33824516/Temporary_Hellfire_and_the_Making_of_Sunni_Orthodoxy_pdf"
            ),
        ],
    ),
    (
        "shia",
        &[
            source!(
                "Jahannam (Wikipedia)",
                "https://en.wikipedia.org/wiki/Jahannam"
            ),
            source!(
                "Twelver Shi'ism",
                "https://en.wikipedia.org/wiki/Twelver_Shi%27ism"
            ),
        ],
    ),
    (
        "ahmadiyya",
        &[
            source!(
                "Life After Death (Ahmadiyya, alislam.org)",
                "https://www.alislam.org/book/study-of-islam/life-after-death/"
            ),
            source!(
                "A Philosophical Explanation of the Doctrine of Hell",
                "https://alahmadiyya.org/articles-magazines-islam-ahmadiyya/the-review-of-religions-english-1902-1914-official-magazine-of-the-ahmadiyya-movement/190803and04/a-philosophical-explanation-of-the-doctrine-of-hell/"
            ),
        ],
    ),
    (
        "orthodox_judaism",
        &[
            source!(
                "Salvation of righteous gentiles (Boston College)",
                "https://www.bc.edu/content/dam/files/research_sites/cjl/texts/cjrelations/resources/sourcebook/Righteousgentiles-salvation.htm"
            ),
            source!(
                "Heaven and hell in Jewish tradition",
                "https://www.myjewishlearning.com/article/heaven-and-hell-in-jewish-tradition/"
            ),
            source!("Shituf (Wikipedia)", "https://en.wikipedia.org/wiki/Shituf"),
        ],
    ),
    (
        "reform_judaism",
        &[source!(
            "Heaven and hell in Jewish tradition",
            "https://www.myjewishlearning.com/article/heaven-and-hell-in-jewish-tradition/"
        )],
    ),
    (
        "advaita",
        &[source!(
            "Advaita Vedanta (Internet Encyclopedia of Philosophy)",
            "https://iep.utm.edu/advaita-vedanta/"
        )],
    ),
    (
        "vaishnava",
        &[
            source!(
                "Gaudiya Vaishnavism (Wikipedia)",
                "https://en.wikipedia.org/wiki/Gaudiya_Vaishnavism"
            ),
            source!(
                "Buddha as an avatar of Vishnu",
                "https://vedapath.app/blog/why-is-buddha-listed-as-an-avatar-of-vishnu-the-most-controversial-entry-in-the-dashavatara-2"
            ),
        ],
    ),
    (
        "dvaita",
        &[
            source!(
                "Tamo-yogyas (Wikipedia)",
                "https://en.wikipedia.org/wiki/Tamo-yogyas"
            ),
            source!(
                "Mukti-yogyas (Wikipedia)",
                "https://en.wikipedia.org/wiki/Mukti-yogyas"
            ),
            source!(
                "Nitya-samsarins (Wikipedia)",
                "https://en.wikipedia.org/wiki/Nitya-samsarins"
            ),
        ],
    ),
    (
        "theravada",
        &[source!(
            "Ubeysekara, The Concept of Rebirth in Theravada Buddhism",
            "https://drarisworld.wordpress.com/2016/12/09/concept-of-rebirth-in-theravada-buddhism-by-dr-ari-ubeysekara/"
        )],
    ),
    (
        "zen",
        &[
            source!(
                "The Lotus Sutra, an overview",
                "https://www.learnreligions.com/the-lotus-sutra-an-overview-450024"
            ),
            source!(
                "Buddha-nature (Wikipedia)",
                "https://en.wikipedia.org/wiki/Buddha-nature"
            ),
        ],
    ),
    (
        "pure_land",
        &[
            source!(
                "Can those who slander the Dharma be reborn in the Pure Land?",
                "https://www.buddhistdoor.net/features/can-those-who-commit-the-five-gravest-transgressions-and-slander-the-right-dharma-be-reborn-in-the-pure-land/"
            ),
            source!(
                "The exclusion clause in the eighteenth vow",
                "https://amida-ji-retreat-temple-romania.blogspot.com/2007/10/exclusion-in-eighteenth-vow.html"
            ),
        ],
    ),
    (
        "vajrayana",
        &[
            source!(
                "Bardo Thodol (Buddha Weekly)",
                "https://buddhaweekly.com/bardo-thodol-tibetan-book-of-the-dead-the-wisdom-that-liberates-from-enlightened-mind-of-guru-rinpoche/"
            ),
            source!("Bardo (Wikipedia)", "https://en.wikipedia.org/wiki/Bardo"),
        ],
    ),
    (
        "nichiren",
        &[
            source!(
                "Nembutsu and the Hell of Incessant Suffering (Nichiren Library)",
                "https://www.nichirenlibrary.org/en/wnd-2/Content/175"
            ),
            source!(
                "Stone, The Sin of Slandering the True Dharma in Nichiren's Thought",
                "https://www.princeton.edu/~jstone/Articles%20on%20the%20Lotus%20Sutra%20Tendai%20and%20Nichiren%20Buddhism/The%20Sin%20of%20Slandering%20the%20True%20Dharma%20in%20Nichiren's%20Thought%20(2012).pdf"
            ),
        ],
    ),
    (
        "jain",
        &[
            source!(
                "Soul (Jainpedia)",
                "https://jainpedia.org/themes/principles/jain-beliefs/soul/"
            ),
            source!(
                "Moksha in Jainism",
                "https://en.wikipedia.org/wiki/Moksha_(Jainism)"
            ),
        ],
    ),
    (
        "sikh",
        &[
            source!(
                "Message of the Guru Granth Sahib",
                "https://en.wikipedia.org/wiki/Message_of_the_Guru_Granth_Sahib"
            ),
            source!(
                "Sikh beliefs about life after death (WJEC)",
                "https://resource.download.wjec.co.uk/vtc/2023-24/mfw/mfw23-24_7-29/pdf/_en/sikhism_a_meaning-of-life-and-life-after-death.pdf"
            ),
        ],
    ),
    (
        "daoism",
        &[
            source!(
                "Xian, Daoist immortal (Britannica)",
                "https://www.britannica.com/topic/xian-Daoism"
            ),
            source!("Diyu (Wikipedia)", "https://en.wikipedia.org/wiki/Diyu"),
        ],
    ),
    (
        "confucianism",
        &[
            source!(
                "Analects, book 11",
                "https://monadnock.net/confucius/analects-11.html"
            ),
            source!(
                "Ancestor veneration in China",
                "https://en.wikipedia.org/wiki/Ancestor_veneration_in_China"
            ),
        ],
    ),
    (
        "chinese_folk",
        &[
            source!("Diyu (Wikipedia)", "https://en.wikipedia.org/wiki/Diyu"),
            source!(
                "Ancestor veneration in China",
                "https://en.wikipedia.org/wiki/Ancestor_veneration_in_China"
            ),
        ],
    ),
    (
        "shinto",
        &[
            source!(
                "Yomi (World History Encyclopedia)",
                "https://www.worldhistory.org/Yomi/"
            ),
            source!(
                "Hirata Atsutane (Wikipedia)",
                "https://en.wikipedia.org/wiki/Hirata_Atsutane"
            ),
        ],
    ),
    (
        "tenrikyo",
        &[
            source!(
                "Passing Away for Rebirth (Tenrikyo)",
                "https://www.tenrikyo.or.jp/en/newsletter/html/tt7/denaoshi.html"
            ),
            source!(
                "Joyous Life (Wikipedia)",
                "https://en.wikipedia.org/wiki/Joyous_Life"
            ),
        ],
    ),
    (
        "caodai",
        &[
            source!(
                "Cao Dai (New World Encyclopedia)",
                "https://www.newworldencyclopedia.org/entry/Cao_Dai"
            ),
            source!(
                "Holy Messages from God, Tay Ninh Holy See",
                "https://daotam.info/booksv/pdf/leminhhoang/thanhngon1.pdf"
            ),
        ],
    ),
    (
        "zoroastrian",
        &[
            source!(
                "Frashokereti (Wikipedia)",
                "https://en.wikipedia.org/wiki/Frashokereti"
            ),
            source!(
                "Chinvat Bridge (Wikipedia)",
                "https://en.wikipedia.org/wiki/Chinvat_Bridge"
            ),
        ],
    ),
    (
        "manichaean",
        &[
            source!(
                "Manichean Eschatology (Encyclopaedia Iranica)",
                "https://www.iranicaonline.org/articles/eschatology-ii/"
            ),
            source!(
                "Manicheism, general survey (Encyclopaedia Iranica)",
                "https://www.iranicaonline.org/articles/manicheism-1-general-survey/"
            ),
        ],
    ),
    (
        "yazidi",
        &[
            source!(
                "Yazidi religious beliefs",
                "https://peacock-angel.org/yazidi.beliefs.htm"
            ),
            source!(
                "What do Iraq's Yazidis believe? (Christianity Today)",
                "https://www.christianitytoday.com/2025/07/yazidis-iraq-kurdistan-reincarnation-melek-tawus-religious-literacy/"
            ),
        ],
    ),
    (
        "druze",
        &[
            source!(
                "Reincarnation and notq among the Druze (Moshe Dayan Center)",
                "https://dayan.org/content/life-death-and-beyond-belief-reincarnation-and-phenomenon-notq-druze-community"
            ),
            source!(
                "The religion with no converts",
                "https://www.amdigital.co.uk/insights/blog/the-druze-and-al-hakim-the-religion-with-no-converts"
            ),
        ],
    ),
    (
        "mandaean",
        &[
            source!(
                "Matarta (Wikipedia)",
                "https://en.wikipedia.org/wiki/Matarta"
            ),
            source!(
                "Masiqta (Wikipedia)",
                "https://en.wikipedia.org/wiki/Masiqta"
            ),
        ],
    ),
    (
        "bahai",
        &[
            source!(
                "Heaven and Hell (bahai.org)",
                "https://www.bahai.org/beliefs/life-spirit/human-soul/heaven-hell"
            ),
            source!(
                "Lights of Guidance, life after death",
                "https://bahai.works/Lights_of_Guidance/Life_after_Death;_the_Soul"
            ),
        ],
    ),
    (
        "gnostic",
        &[
            source!(
                "Valentinianism (Wikipedia)",
                "https://en.wikipedia.org/wiki/Valentinianism"
            ),
            source!(
                "Hylic, psychic and pneumatic",
                "https://spiritualseek.com/blog/the-three-distinct-types-of-human-beings-in-gnosticism-hylic-psychic-and-pneumatic/"
            ),
        ],
    ),
    (
        "cathar",
        &[
            source!(
                "Consolamentum (Wikipedia)",
                "https://en.wikipedia.org/wiki/Consolamentum"
            ),
            source!(
                "Cathars (World History Encyclopedia)",
                "https://www.worldhistory.org/Cathars/"
            ),
        ],
    ),
    (
        "egyptian",
        &[
            source!(
                "The Egyptian afterlife and the feather of truth",
                "https://www.worldhistory.org/article/42/the-egyptian-afterlife--the-feather-of-truth/"
            ),
            source!(
                "The negative confession, Book of the Dead spell 125",
                "https://ancientegyptonline.co.uk/negativeconfession/"
            ),
        ],
    ),
    (
        "mesopotamian",
        &[
            source!(
                "Ancient Mesopotamian beliefs in the afterlife",
                "https://www.worldhistory.org/article/701/ancient-mesopotamian-beliefs-in-the-afterlife/"
            ),
            source!("Kispu (Wikipedia)", "https://en.wikipedia.org/wiki/Kispu"),
        ],
    ),
    (
        "hellenic",
        &[
            source!(
                "Asphodel Meadows (Wikipedia)",
                "https://en.wikipedia.org/wiki/Asphodel_Meadows"
            ),
            source!(
                "Elysium (Theoi)",
                "https://www.theoi.com/Kosmos/Elysion.html"
            ),
        ],
    ),
    (
        "norse",
        &[
            source!(
                "Death and the afterlife (Norse Mythology for Smart People)",
                "https://norse-mythology.org/concepts/death-and-the-afterlife/"
            ),
            source!(
                "Nastrond (Wikipedia)",
                "https://en.wikipedia.org/wiki/N%C3%A1str%C3%B6nd"
            ),
        ],
    ),
    (
        "aztec",
        &[
            source!(
                "Mictlan and its inhabitants (Mexicolore)",
                "https://www.mexicolore.co.uk/aztecs/underworld/mictlan-and-its-inhabitants"
            ),
            source!(
                "What did ordinary Aztec people turn into? (Mexicolore)",
                "https://www.mexicolore.co.uk/aztecs/ask-experts/what-did-ordinary-people-turn-into-in-the-afterlife"
            ),
        ],
    ),
    (
        "yoruba",
        &[
            source!(
                "Unravelling the Yoruba belief of the afterlife",
                "https://oriire.com/article/journeying-beyond-unraveling-the-yoruba-belief-of-the-afterlife"
            ),
            source!(
                "Belief in reincarnation among the Akure Yoruba",
                "https://file.scirp.org/Html/7-1650647_63610.htm"
            ),
        ],
    ),
    (
        "spiritism",
        &[
            source!(
                "Kardec, The Spirits' Book",
                "https://cei-spiritistcouncil.com/wp-content/uploads/2020/07/2-The-Spirits-Book.pdf"
            ),
            source!(
                "Spiritism (Wikipedia)",
                "https://en.wikipedia.org/wiki/Spiritism"
            ),
        ],
    ),
    (
        "scientology",
        &[
            source!(
                "Scientology beliefs and practices",
                "https://en.wikipedia.org/wiki/Scientology_beliefs_and_practices"
            ),
            source!(
                "Operating Thetan (Wikipedia)",
                "https://en.wikipedia.org/wiki/Operating_Thetan"
            ),
        ],
    ),
    (
        "wicca",
        &[
            source!(
                "What is the Summerland in Wiccan belief?",
                "https://www.learnreligions.com/what-is-the-summerland-2562874"
            ),
            source!("Wicca (Wikipedia)", "https://en.wikipedia.org/wiki/Wicca"),
        ],
    ),
];

/// The sources behind one tradition's row.
pub fn sources(id: &str) -> &'static [Source] {
    SOURCES
        .iter()
        .find(|(entry, _)| *entry == id)
        .map(|(_, sources)| *sources)
        .unwrap_or(&[])
}

/// The prior the table opens on: naturalism at ninety percent, and the
/// remaining ten split between the rest in proportion to their followers,
/// on the reasoning that if a religion is true it is more likely to be one
/// many people hold than one nobody does.
pub fn default_priors() -> Vec<f64> {
    let naturalism = 90.0;
    let rest: f64 = RELIGIONS
        .iter()
        .filter(|religion| religion.id != "atheism")
        .map(|religion| religion.adherents)
        .sum();

    RELIGIONS
        .iter()
        .map(|religion| {
            if religion.id == "atheism" {
                naturalism
            } else {
                religion.adherents * (100.0 - naturalism) / rest
            }
        })
        .collect()
}
/// What one tradition says becomes of a follower of another.
#[derive(Clone, Debug, PartialEq)]
pub struct Outcome {
    /// Coefficient on infinity, in `[-1, 1]`.
    pub eternal: f64,
    /// Everything that ends, in lifetimes.
    pub finite: f64,
    /// The teaching the verdict is based on.
    pub note: &'static str,
}

impl Outcome {
    /// Paradise without end.
    const fn heaven(finite: f64, note: &'static str) -> Self {
        Outcome {
            eternal: 1.0,
            finite,
            note,
        }
    }

    /// Torment without end.
    const fn hell(finite: f64, note: &'static str) -> Self {
        Outcome {
            eternal: -1.0,
            finite,
            note,
        }
    }

    /// Nothing that lasts forever: annihilation, or an afterlife that ends.
    const fn mortal(finite: f64, note: &'static str) -> Self {
        Outcome {
            eternal: 0.0,
            finite,
            note,
        }
    }

    /// An eternity of a given intensity, or a tradition's own odds of one.
    const fn eternal(eternal: f64, finite: f64, note: &'static str) -> Self {
        Outcome {
            eternal,
            finite,
            note,
        }
    }

    /// The tradition gives this person a chance of paradise and a chance of
    /// hell, and the rest of the probability to whatever `finite` describes.
    const fn odds(heaven: f64, hell: f64, finite: f64, note: &'static str) -> Self {
        Outcome {
            eternal: heaven - hell,
            finite,
            note,
        }
    }
}

macro_rules! religion {
    (
        $id:literal, $name:literal, $family:ident, $theism:ident,
        idols: $idols:literal, cremates: $cremates:literal, vegetarian: $veg:literal,
        martial: $martial:literal, sacrifice: $sacrifice:literal,
        cost: $cost:literal, adherents: $adherents:literal,
        $summary:literal
    ) => {
        Religion {
            id: $id,
            name: $name,
            family: Family::$family,
            theism: Theism::$theism,
            idols: $idols,
            cremates: $cremates,
            vegetarian: $veg,
            martial: $martial,
            sacrifice: $sacrifice,
            cost: $cost,
            adherents: $adherents,
            summary: $summary,
        }
    };
}

/// Every tradition in the table, in the order the table shows them. The
/// adherent shares are from the Pew Research Center's 2020 estimates where
/// Pew counts a group, and rough headcounts against eight billion elsewhere;
/// the splits inside Christianity, Islam, Hinduism and Buddhism are the
/// author's, and a tradition with no followers left is given a small positive share.
pub const RELIGIONS: &[Religion] = &[
    religion!("atheism", "atheism (naturalism)", Philosophical, Atheist,
        idols: false, cremates: true, vegetarian: false, martial: false, sacrifice: false,
        cost: 0.0, adherents: 23.7,
        "Death is the end of the person. Nothing follows."),
    religion!("deism", "deism", Philosophical, Deist,
        idols: false, cremates: false, vegetarian: false, martial: false, sacrifice: false,
        cost: 0.0, adherents: 0.3,
        "A creator who does not intervene; whether anything follows death is judged by conduct, if at all."),
    religion!("pantheism", "pantheism (Spinoza, Stoicism)", Philosophical, Pantheist,
        idols: false, cremates: true, vegetarian: false, martial: false, sacrifice: false,
        cost: 0.0, adherents: 0.2,
        "God is nature; the person dissolves back into it."),
    religion!("catholic", "Roman Catholicism", Christian, Monotheist,
        idols: false, cremates: false, vegetarian: false, martial: false, sacrifice: false,
        cost: 0.05, adherents: 17.4,
        "Heaven, purgatory first for most, or eternal hell for those who die in unrepented mortal sin."),
    religion!("orthodox", "Eastern Orthodoxy", Christian, Monotheist,
        idols: false, cremates: false, vegetarian: false, martial: false, sacrifice: false,
        cost: 0.06, adherents: 3.2,
        "Theosis, union with God, or God's presence experienced as fire by those who refused it, without end."),
    religion!("mainline", "mainline Protestantism", Christian, Monotheist,
        idols: false, cremates: false, vegetarian: false, martial: false, sacrifice: false,
        cost: 0.03, adherents: 2.4,
        "Salvation by grace through faith; heaven, or a hell many of its theologians hope is empty."),
    religion!("evangelical", "evangelical Protestantism", Christian, Monotheist,
        idols: false, cremates: false, vegetarian: false, martial: false, sacrifice: false,
        cost: 0.06, adherents: 4.4,
        "Personal faith in Christ, or eternal hell."),
    religion!("calvinist", "Calvinism (Reformed)", Christian, Monotheist,
        idols: false, cremates: false, vegetarian: false, martial: false, sacrifice: false,
        cost: 0.05, adherents: 0.8,
        "Heaven for the elect, hell for the reprobate, chosen before the world was made."),
    religion!("lds", "Latter-day Saints (Mormonism)", Christian, Monotheist,
        idols: false, cremates: false, vegetarian: false, martial: false, sacrifice: false,
        cost: 0.15, adherents: 0.21,
        "Three kingdoms of glory for nearly everyone; outer darkness only for those who knew and denied."),
    religion!("jw", "Jehovah's Witnesses", Christian, Monotheist,
        idols: false, cremates: false, vegetarian: false, martial: false, sacrifice: false,
        cost: 0.2, adherents: 0.11,
        "Everlasting life on a paradise earth, or destruction; there is no hell of torment."),
    religion!("adventist", "Seventh-day Adventism", Christian, Monotheist,
        idols: false, cremates: false, vegetarian: true, martial: false, sacrifice: false,
        cost: 0.08, adherents: 0.28,
        "Sleep until the resurrection; eternal life for the saved, and the lost are burned up, not tormented forever."),
    religion!("universalist", "Christian universalism", Christian, Monotheist,
        idols: false, cremates: false, vegetarian: false, martial: false, sacrifice: false,
        cost: 0.02, adherents: 0.02,
        "All are reconciled to God in the end; hell is a purifying fire that empties."),
    religion!("sunni", "Sunni Islam", Islamic, Monotheist,
        idols: false, cremates: false, vegetarian: false, martial: false, sacrifice: false,
        cost: 0.1, adherents: 22.1,
        "Paradise for Muslims, after the Fire has burned off their sins; the Fire without end for those who rejected the message."),
    religion!("shia", "Shia Islam (Twelver)", Islamic, Monotheist,
        idols: false, cremates: false, vegetarian: false, martial: false, sacrifice: false,
        cost: 0.1, adherents: 3.35,
        "As Sunni Islam, with the Imams as the way, and the intercession of the Prophet's house."),
    religion!("ahmadiyya", "Ahmadiyya Islam", Islamic, Monotheist,
        idols: false, cremates: false, vegetarian: false, martial: false, sacrifice: false,
        cost: 0.1, adherents: 0.15,
        "Hell is reformatory and temporary: everyone in it is released once reformed, and everyone ends in paradise."),
    religion!("orthodox_judaism", "Orthodox Judaism", Jewish, Monotheist,
        idols: false, cremates: false, vegetarian: false, martial: false, sacrifice: false,
        cost: 0.15, adherents: 0.06,
        "Gehinnom for at most twelve months, then the World to Come for all Israel and the righteous of every nation."),
    religion!("reform_judaism", "Reform Judaism", Jewish, Monotheist,
        idols: false, cremates: false, vegetarian: false, martial: false, sacrifice: false,
        cost: 0.03, adherents: 0.14,
        "The soul is immortal and the World to Come is open to the righteous of every people."),
    religion!("advaita", "Advaita Vedanta (Hinduism)", Hindu, Pantheist,
        idols: true, cremates: true, vegetarian: true, martial: false, sacrifice: false,
        cost: 0.05, adherents: 5.5,
        "Rebirth according to karma until the self is known to be Brahman; every soul gets there."),
    religion!("vaishnava", "Gaudiya Vaishnavism (Hinduism)", Hindu, Monotheist,
        idols: true, cremates: true, vegetarian: true, martial: false, sacrifice: false,
        cost: 0.1, adherents: 9.0,
        "Devotion to Krishna ends in his eternal abode; everyone else keeps taking birth, in hells that end and heavens that end."),
    religion!("dvaita", "Dvaita Vedanta (Hinduism)", Hindu, Monotheist,
        idols: true, cremates: true, vegetarian: true, martial: false, sacrifice: false,
        cost: 0.08, adherents: 0.4,
        "Souls are of three kinds by nature: fit for liberation, bound to wander forever, or bound for eternal darkness."),
    religion!("theravada", "Theravada Buddhism", Buddhist, Nontheist,
        idols: true, cremates: true, vegetarian: false, martial: false, sacrifice: false,
        cost: 0.06, adherents: 1.6,
        "Rebirth according to kamma until nibbana, which only the Noble Eightfold Path reaches."),
    religion!("zen", "Mahayana Buddhism (Zen, Chan)", Buddhist, Nontheist,
        idols: true, cremates: true, vegetarian: true, martial: false, sacrifice: false,
        cost: 0.05, adherents: 1.4,
        "Every being has Buddha-nature and every being will awaken; the bodhisattvas stay until the last one does."),
    religion!("pure_land", "Pure Land Buddhism", Buddhist, Nontheist,
        idols: true, cremates: true, vegetarian: false, martial: false, sacrifice: false,
        cost: 0.02, adherents: 0.7,
        "Call Amida's name in faith and be born in the Pure Land, where Buddhahood is certain."),
    religion!("vajrayana", "Vajrayana Buddhism (Tibetan)", Buddhist, Nontheist,
        idols: true, cremates: true, vegetarian: false, martial: false, sacrifice: false,
        cost: 0.08, adherents: 0.25,
        "The bardo between lives, then rebirth, or liberation in this body for the practitioner who recognises the clear light."),
    religion!("nichiren", "Nichiren Buddhism", Buddhist, Nontheist,
        idols: false, cremates: true, vegetarian: false, martial: false, sacrifice: false,
        cost: 0.04, adherents: 0.15,
        "Chanting the Lotus Sutra's title opens Buddhahood in this life; slandering the sutra opens the hell of incessant suffering."),
    religion!("jain", "Jainism", Jain, Nontheist,
        idols: true, cremates: true, vegetarian: true, martial: false, sacrifice: false,
        cost: 0.2, adherents: 0.06,
        "Rebirth by the weight of karma, and harm is the heaviest karma; liberation for the soul that sheds it all."),
    religion!("sikh", "Sikhism", Sikh, Monotheist,
        idols: false, cremates: true, vegetarian: false, martial: true, sacrifice: false,
        cost: 0.06, adherents: 0.35,
        "Union with the One through the Name and grace; otherwise the round of 8.4 million births continues."),
    religion!("daoism", "religious Daoism", EastAsian, Polytheist,
        idols: true, cremates: false, vegetarian: false, martial: false, sacrifice: false,
        cost: 0.05, adherents: 0.12,
        "Immortality for the adept; for everyone else the courts of the underworld, then a new body."),
    religion!("confucianism", "Confucianism", EastAsian, Nontheist,
        idols: false, cremates: false, vegetarian: false, martial: false, sacrifice: false,
        cost: 0.03, adherents: 0.05,
        "'Not yet understanding life, how could you understand death?' What lasts is your name and your descendants' rites."),
    religion!("chinese_folk", "Chinese folk religion", EastAsian, Polytheist,
        idols: true, cremates: false, vegetarian: false, martial: false, sacrifice: false,
        cost: 0.03, adherents: 0.75,
        "The ten courts of Diyu punish each sin in proportion, the family's offerings ease the stay, and then you are reborn."),
    religion!("shinto", "Shinto", EastAsian, Polytheist,
        idols: false, cremates: true, vegetarian: false, martial: false, sacrifice: false,
        cost: 0.02, adherents: 0.18,
        "Yomi, the dim land of the dead, and enshrinement as an ancestral kami honoured by the household."),
    religion!("tenrikyo", "Tenrikyo", EastAsian, Monotheist,
        idols: false, cremates: true, vegetarian: false, martial: false, sacrifice: false,
        cost: 0.04, adherents: 0.02,
        "God the Parent takes the borrowed body back and lends another; there is no hell."),
    religion!("caodai", "Cao Dai", EastAsian, Monotheist,
        idols: false, cremates: false, vegetarian: true, martial: false, sacrifice: false,
        cost: 0.05, adherents: 0.05,
        "Rebirth by karma, with the Third Universal Amnesty promising every soul a return to God."),
    religion!("zoroastrian", "Zoroastrianism", Iranian, Monotheist,
        idols: false, cremates: false, vegetarian: false, martial: false, sacrifice: false,
        cost: 0.06, adherents: 0.002,
        "The Chinvat bridge judges deeds; heaven or hell until the Renovation, when every soul is purified and hell ends."),
    religion!("manichaean", "Manichaeism", Gnostic, Dualist,
        idols: false, cremates: false, vegetarian: true, martial: false, sacrifice: false,
        cost: 0.15, adherents: 0.0001,
        "The Elect return to the Light; the rest are reborn until they do, and what clings to darkness at the end is sealed in with it."),
    religion!("yazidi", "Yazidism", Iranian, Monotheist,
        idols: false, cremates: false, vegetarian: false, martial: false, sacrifice: false,
        cost: 0.05, adherents: 0.01,
        "Judged at the bridge, the pure go to heaven and the rest are reborn; the Peacock Angel put out hell's fires with his tears."),
    religion!("druze", "Druze faith", Iranian, Monotheist,
        idols: false, cremates: false, vegetarian: false, martial: false, sacrifice: false,
        cost: 0.05, adherents: 0.015,
        "Immediate rebirth as another Druze until the soul is ready for the Cosmic Mind; the call to join closed in 1043."),
    religion!("mandaean", "Mandaeism", Gnostic, Monotheist,
        idols: false, cremates: false, vegetarian: false, martial: false, sacrifice: false,
        cost: 0.08, adherents: 0.001,
        "The soul climbs through the watch-houses, purified in each, to the World of Light."),
    religion!("bahai", "Baháʼí Faith", Iranian, Monotheist,
        idols: false, cremates: false, vegetarian: false, martial: false, sacrifice: false,
        cost: 0.06, adherents: 0.1,
        "The soul progresses through the worlds of God forever; heaven and hell are nearness and distance, not places."),
    religion!("gnostic", "Gnosticism (Sethian, Valentinian)", Gnostic, Dualist,
        idols: false, cremates: false, vegetarian: false, martial: false, sacrifice: false,
        cost: 0.1, adherents: 0.001,
        "The spark that knows itself escapes the Demiurge's world to the Pleroma; faith without knowledge rests below it; matter returns to matter."),
    religion!("cathar", "Catharism", Gnostic, Dualist,
        idols: false, cremates: false, vegetarian: true, martial: false, sacrifice: false,
        cost: 0.1, adherents: 0.0001,
        "Every soul is a fallen angel; the consolamentum frees it now, and every other one will be freed after enough bodies."),
    religion!("egyptian", "ancient Egyptian religion", Ancient, Polytheist,
        idols: true, cremates: false, vegetarian: false, martial: false, sacrifice: true,
        cost: 0.1, adherents: 0.0001,
        "The heart is weighed against the feather; the justified reach the Field of Reeds forever, and Ammit eats the rest."),
    religion!("mesopotamian", "Mesopotamian religion", Ancient, Polytheist,
        idols: true, cremates: false, vegetarian: false, martial: false, sacrifice: true,
        cost: 0.05, adherents: 0.0001,
        "The house of dust for everyone, king and slave alike, forever; the offerings of descendants are the only comfort."),
    religion!("hellenic", "Greco-Roman religion", Ancient, Polytheist,
        idols: true, cremates: true, vegetarian: false, martial: false, sacrifice: true,
        cost: 0.03, adherents: 0.005,
        "The asphodel meadows for most, Elysium for the heroic and the initiated, Tartarus for those who offended the gods."),
    religion!("norse", "Norse religion", Ancient, Polytheist,
        idols: true, cremates: true, vegetarian: false, martial: true, sacrifice: true,
        cost: 0.1, adherents: 0.005,
        "Valhalla or Fólkvangr for the brave slain, Hel for the rest, all of it until Ragnarök."),
    religion!("aztec", "Aztec (Mexica) religion", Ancient, Polytheist,
        idols: true, cremates: true, vegetarian: false, martial: true, sacrifice: true,
        cost: 0.3, adherents: 0.0001,
        "The afterlife goes by the manner of death, not by belief, and for most it is a four-year road through Mictlan to extinction."),
    religion!("yoruba", "Yoruba religion (Ifá, Orisha)", African, Polytheist,
        idols: true, cremates: false, vegetarian: false, martial: false, sacrifice: true,
        cost: 0.05, adherents: 0.4,
        "Olodumare judges character; the good rest in òrun rere and are reborn into their own line, the wicked go to the potsherd heaven."),
    religion!("spiritism", "Spiritism (Kardec)", NewReligious, Monotheist,
        idols: false, cremates: false, vegetarian: false, martial: false, sacrifice: false,
        cost: 0.02, adherents: 0.08,
        "Every spirit progresses through as many lives as it needs; there is no hell and no eternal punishment."),
    religion!("scientology", "Scientology", NewReligious, Nontheist,
        idols: false, cremates: true, vegetarian: false, martial: false, sacrifice: false,
        cost: 0.6, adherents: 0.001,
        "The thetan is immortal and is reborn; Operating Thetans leave the cycle of implants, everyone else stays on the whole track."),
    religion!("wicca", "Wicca", NewReligious, Polytheist,
        idols: true, cremates: true, vegetarian: false, martial: false, sacrifice: false,
        cost: 0.02, adherents: 0.02,
        "Rest in the Summerland, then rebirth; there is no hell."),
];

/// What `truth`, taken as the true account of the world, says becomes of
/// someone who spent their life following `followed`.
pub fn verdict(truth: &Religion, followed: &Religion) -> Outcome {
    let x = followed;
    match truth.id {
        "atheism" => Outcome::mortal(
            0.0,
            "Nothing happens. Death is the end, whatever you followed, and the only difference between the columns is what the practice cost you while you were alive.",
        ),
        "deism" => Outcome::eternal(
            0.5,
            0.0,
            "A creator who never sent a revelation cannot hold you to one; if there is a hereafter it goes by conduct, and we assumed the same conduct in every column. Paine hoped for happiness beyond this life and would not say more.",
        ),
        "pantheism" => Outcome::mortal(
            0.0,
            "You return to the whole you never left. Spinoza's eternity of the mind is not personal survival; no one remains to be rewarded.",
        ),
        "catholic" => verdict_catholic(x),
        "orthodox" => verdict_orthodox(x),
        "mainline" => verdict_mainline(x),
        "evangelical" => verdict_evangelical(x),
        "calvinist" => verdict_calvinist(x),
        "lds" => verdict_lds(x),
        "jw" => verdict_jw(x),
        "adventist" => verdict_adventist(x),
        "universalist" => {
            if x.christian() {
                Outcome::heaven(
                    0.0,
                    "All are reconciled to God in the end (1 Corinthians 15:22, Colossians 1:20), and you already believed it.",
                )
            } else {
                Outcome::heaven(
                    -5.0,
                    "All are reconciled to God in the end (1 Corinthians 15:22, Colossians 1:20); the fire of 1 Corinthians 3:15 purifies rather than punishes, and you have some of it to go through first.",
                )
            }
        }
        "sunni" => verdict_sunni(x),
        "shia" => verdict_shia(x),
        "ahmadiyya" => {
            if x.is("ahmadiyya") {
                Outcome::heaven(
                    0.0,
                    "Paradise, having accepted the Promised Messiah; hell is reformatory and everyone eventually leaves it, and you have little to reform.",
                )
            } else if x.muslim() {
                Outcome::heaven(
                    -2.0,
                    "A Muslim who rejected the Promised Messiah is still a Muslim; some time in the reformatory, then paradise.",
                )
            } else {
                Outcome::heaven(
                    -20.0,
                    "Hell reforms rather than punishes. Mirza Ghulam Ahmad taught that its fire goes out for each soul once the soul is cured, reading 'except as your Lord wills' in Qur'an 11:107 as a promise; everyone ends in paradise, and disbelief means a long time there first.",
                )
            }
        }
        "orthodox_judaism" => verdict_orthodox_judaism(x),
        "reform_judaism" => Outcome::eternal(
            0.85,
            0.0,
            "Reform Judaism keeps the immortality of the soul and drops the rest: the World to Come is open to the righteous of every people, and nobody is tormented for a creed. What it does not promise is any detail.",
        ),
        "advaita" => verdict_advaita(x),
        "vaishnava" => verdict_vaishnava(x),
        "dvaita" => verdict_dvaita(x),
        "theravada" => verdict_theravada(x),
        "zen" => verdict_zen(x),
        "pure_land" => verdict_pure_land(x),
        "vajrayana" => verdict_vajrayana(x),
        "nichiren" => verdict_nichiren(x),
        "jain" => verdict_jain(x),
        "sikh" => verdict_sikh(x),
        "daoism" => verdict_daoism(x),
        "confucianism" => {
            if x.family == Family::EastAsian {
                Outcome::mortal(
                    0.5,
                    "The Master would not speak of spirits; what continues is your name, your line, and the rites your descendants perform at your tablet, which you saw to.",
                )
            } else {
                Outcome::mortal(
                    0.0,
                    "The Master would not speak of spirits (Analects 11.12); what continues is your name and your line, and nobody keeps your tablet.",
                )
            }
        }
        "chinese_folk" => verdict_chinese_folk(x),
        "shinto" => {
            if x.is("shinto") {
                Outcome::eternal(
                    0.1,
                    0.0,
                    "Yomi is dim and unclean, but the dead are also enshrined as household kami, fed at the kamidana and honoured at Obon; Hirata Atsutane held that the soul stays close to its own people.",
                )
            } else {
                Outcome::eternal(
                    -0.05,
                    0.0,
                    "Yomi, the land of the dead that Izanagi fled from, with nobody performing the rites that would make you an ancestor worth calling on.",
                )
            }
        }
        "tenrikyo" => {
            if x.is("tenrikyo") {
                Outcome::mortal(
                    2.0,
                    "God the Parent lends you a body, takes it back, and lends you another; there is no hell, and you spent this life building the Joyous Life, which is what the teaching asks.",
                )
            } else {
                Outcome::mortal(
                    1.0,
                    "God the Parent lends you a body, takes it back, and lends you another; there is no hell, only the Joyous Life to be built here, which you helped with in your own way.",
                )
            }
        }
        "caodai" => {
            if x.is("caodai") {
                Outcome::heaven(
                    -5.0,
                    "The Third Universal Amnesty promises every soul a return to God; the Cao Dai practices shorten the series of rebirths.",
                )
            } else {
                Outcome::heaven(
                    -10.0,
                    "The Third Universal Amnesty promises every soul a return to God, and Cao Dai counts your religion among the earlier amnesties; your series of rebirths is longer and ends in the same place.",
                )
            }
        }
        "zoroastrian" => verdict_zoroastrian(x),
        "manichaean" => verdict_manichaean(x),
        "yazidi" => {
            if x.is("yazidi") {
                Outcome::eternal(
                    0.9,
                    -3.0,
                    "Judged by Sheikh Adi at the bridge; the pure go to heaven and the rest are reborn as another Yazidi, which the tradition calls changing the shirt. Hell's fires were put out by the Peacock Angel's tears.",
                )
            } else {
                Outcome::eternal(
                    0.5,
                    -10.0,
                    "There is no hell. But heaven is reached only through Yazidi births, and one cannot become a Yazidi, only be born one; deeds still count, and the tradition does not say much about anyone else.",
                )
            }
        }
        "druze" => {
            if x.is("druze") {
                Outcome::eternal(
                    0.9,
                    -5.0,
                    "Reborn at once as another Druze, life after life, until the soul is ready to join the Cosmic Mind; the faithful are rewarded when al-Hakim returns.",
                )
            } else {
                Outcome::odds(
                    0.3,
                    0.2,
                    -10.0,
                    "The call to the unitarian faith opened in 1017 and closed in 1043, and a soul that did not answer then is reborn in its own community. On the Day it stands outside the faith; the esoteric texts are not read to outsiders, so what that costs is not said.",
                )
            }
        }
        "mandaean" => verdict_mandaean(x),
        "bahai" => {
            if x.is("bahai") {
                Outcome::heaven(
                    0.0,
                    "Recognising the Manifestation of God for this age is the first duty; the soul then progresses through the worlds of God forever.",
                )
            } else if x.godless() {
                Outcome::heaven(
                    -6.0,
                    "Distance from God is what hell means and it is a condition, not a sentence; the soul still progresses after death, and the prayers of others help it along.",
                )
            } else {
                Outcome::heaven(
                    -3.0,
                    "Christ, Muhammad, Krishna, the Buddha and Zoroaster were all Manifestations of the one God, and you followed one from an earlier age; the soul progresses after death, a little more slowly for not having recognised the latest.",
                )
            }
        }
        "gnostic" => verdict_gnostic(x),
        "cathar" => {
            if x.is("cathar") {
                Outcome::heaven(
                    -3.0,
                    "The consolamentum at the end of life frees the spirit from the prince of this world and returns it to the good God.",
                )
            } else if x.is("catholic") {
                Outcome::heaven(
                    -35.0,
                    "Every soul is a fallen angel and every one will return to the good God after enough bodies; the Roman Church is the church of the god who made this world, so it did not help.",
                )
            } else {
                Outcome::heaven(
                    -30.0,
                    "Every soul is a fallen angel and every one will return to the good God after enough bodies; yours has a few to go.",
                )
            }
        }
        "egyptian" => verdict_egyptian(x),
        "mesopotamian" => verdict_mesopotamian(x),
        "hellenic" => verdict_hellenic(x),
        "norse" => verdict_norse(x),
        "aztec" => {
            if x.is("aztec") {
                Outcome::mortal(
                    0.0,
                    "Where you go depends on how you died, not what you believed. Following this religion raised the odds of dying in a way the sun rewards, and the road through Mictlan is the same four years for everyone else.",
                )
            } else {
                Outcome::mortal(
                    -1.0,
                    "Where you go depends on how you died, not what you believed: warriors, sacrifices and women dead in childbirth go with the sun, the drowned to Tlalocan, and the rest walk four years through Mictlan to be extinguished.",
                )
            }
        }
        "yoruba" => {
            if x.is("yoruba") {
                Outcome::eternal(
                    0.9,
                    0.0,
                    "Olodumare judges ìwà, character, and yours was good; rest in òrun rere, with your orí aligned and the ancestors propitiated, and come back as a grandchild in your own line.",
                )
            } else {
                Outcome::eternal(
                    0.8,
                    0.0,
                    "Olodumare judges ìwà, character, not creed; the good rest in òrun rere and return to their family line, the wicked go to òrun apadi, the heaven of potsherds. You were decent.",
                )
            }
        }
        "spiritism" => {
            if x.is("spiritism") {
                Outcome::heaven(
                    -5.0,
                    "Every spirit progresses through as many lives as it needs; there is no hell and no eternal punishment, and knowing that speeds it up.",
                )
            } else {
                Outcome::heaven(
                    -10.0,
                    "Every spirit progresses through as many lives as it needs; there is no hell and no eternal punishment (The Spirits' Book, questions 1004 to 1009). Every spirit arrives eventually.",
                )
            }
        }
        "scientology" => {
            if x.is("scientology") {
                Outcome::eternal(
                    0.5,
                    -1.0,
                    "Clear, and then Operating Thetan: free of the reactive mind, able to leave the body at will, if you finished the Bridge. Most do not.",
                )
            } else {
                Outcome::eternal(
                    0.0,
                    -5.0,
                    "You are an immortal thetan too, but between lives you are implanted with pictures and sent back; the whole track goes on with no exit.",
                )
            }
        }
        "wicca" => {
            if x.is("wicca") {
                Outcome::mortal(
                    3.0,
                    "Rest in the Summerland, then return; there is no hell, the Rede judges harm and not creed, and you knew this.",
                )
            } else {
                Outcome::mortal(
                    2.0,
                    "Rest in the Summerland, then return; there is no hell, and the Rede judges harm, not creed.",
                )
            }
        }
        _ => Outcome::mortal(0.0, "No verdict recorded."),
    }
}

fn verdict_catholic(x: &Religion) -> Outcome {
    if x.is("catholic") {
        Outcome::odds(
            0.9,
            0.1,
            -3.0,
            "Baptised, with the sacraments and confession available to the end: heaven, after purgatory, unless you die in unrepented mortal sin.",
        )
    } else if x.is("orthodox") {
        Outcome::odds(
            0.85,
            0.15,
            -3.0,
            "Valid sacraments and apostolic succession; the schism is not held against the individual (Unitatis Redintegratio 15).",
        )
    } else if x.creedal_christian() {
        Outcome::odds(
            0.7,
            0.3,
            -4.0,
            "'Justified by faith in baptism, incorporated into Christ' (Unitatis Redintegratio 3), though without confession and the Eucharist, and with more to burn off in purgatory.",
        )
    } else if x.christian() {
        Outcome::odds(
            0.4,
            0.6,
            -5.0,
            "The Church does not recognise this baptism as Christian (the 2001 ruling on Latter-day Saint baptism; Witnesses likewise); salvation only through invincible ignorance and a good conscience, as for anyone outside.",
        )
    } else if x.jewish() || x.muslim() {
        Outcome::odds(
            0.45,
            0.55,
            -5.0,
            "Lumen Gentium 16 names the people of Israel and 'the Muslims, who profess to hold the faith of Abraham' first among those outside the Church who may be saved, if they sincerely seek God and follow their conscience. Whoever knows the Church is necessary and refuses to enter cannot be (Lumen Gentium 14).",
        )
    } else if x.godless() {
        Outcome::odds(
            0.3,
            0.7,
            -5.0,
            "Lumen Gentium 16 allows salvation for those who 'without blame on their part have not yet arrived at an explicit knowledge of God'; the Catechism (2125) calls the deliberate rejection of God a sin against the first commandment.",
        )
    } else {
        Outcome::odds(
            0.35,
            0.65,
            -5.0,
            "Salvation is possible for those who through no fault of their own do not know the Gospel and follow their conscience (Lumen Gentium 16); whoever knows the Church is necessary and refuses to enter cannot be saved (Lumen Gentium 14). The Church does not say which of the two describes any particular person.",
        )
    }
}

fn verdict_orthodox(x: &Religion) -> Outcome {
    if x.is("orthodox") {
        Outcome::odds(
            0.9,
            0.1,
            -2.0,
            "Baptised, chrismated, communed and confessed in the Church; theosis, unless you turned away at the end. The Church prays for you for forty days and then on.",
        )
    } else if x.is("catholic") {
        Outcome::odds(
            0.8,
            0.2,
            -2.0,
            "Baptised in the Trinity, heterodox in the Filioque and the papacy; 'we know where the Church is, we do not know where it is not.'",
        )
    } else if x.creedal_christian() {
        Outcome::odds(
            0.65,
            0.35,
            -2.0,
            "Baptised in the Trinity, without the mysteries; the Church does not pronounce on those outside it, and hopes.",
        )
    } else if x.christian() {
        Outcome::odds(
            0.35,
            0.65,
            -3.0,
            "Not baptised in the Trinity as the Church understands it; outside, with whatever mercy God shows to those outside.",
        )
    } else if x.godless() {
        Outcome::odds(
            0.25,
            0.75,
            -3.0,
            "God's love is one fire for everyone; the saints feel it as light and those who refused God feel it as burning. The Church does not say who is which, and does not condemn anyone by name.",
        )
    } else {
        Outcome::odds(
            0.35,
            0.65,
            -3.0,
            "God's love is one fire for everyone; the saints feel it as light and those who refused God feel it as burning. The Church does not say who is which, and does not condemn anyone by name.",
        )
    }
}

fn verdict_mainline(x: &Religion) -> Outcome {
    if x.creedal_christian() {
        Outcome::odds(
            0.85,
            0.15,
            0.0,
            "Saved by grace through faith in Christ, whichever church you attended.",
        )
    } else if x.christian() {
        Outcome::odds(
            0.5,
            0.5,
            0.0,
            "Faith in a Christ the creeds would not recognise; grace is wider than the creeds, most mainline theologians say, and how much wider they do not say.",
        )
    } else if x.godless() {
        Outcome::odds(
            0.3,
            0.7,
            0.0,
            "Hell is real and the churches hope it is nearly empty; God's grace may reach those who never knew Christ, and the church does not say how.",
        )
    } else {
        Outcome::odds(
            0.4,
            0.6,
            0.0,
            "God's grace may reach those who never knew Christ; the church does not say how, and hopes. Karl Rahner called you an anonymous Christian.",
        )
    }
}

fn verdict_evangelical(x: &Religion) -> Outcome {
    if x.is("evangelical") {
        Outcome::odds(
            0.95,
            0.05,
            0.0,
            "You confessed Jesus as Lord and believed God raised him from the dead (Romans 10:9). Saved, unless the confession was not sincere.",
        )
    } else if x.any(&["mainline", "calvinist", "adventist", "universalist"]) {
        Outcome::odds(
            0.85,
            0.15,
            0.0,
            "Trusts in Christ alone; the differences are about church order and the timetable, not the gospel.",
        )
    } else if x.any(&["catholic", "orthodox"]) {
        Outcome::odds(
            0.5,
            0.5,
            0.0,
            "Many evangelicals hold that trusting in sacraments and works is not saving faith; many others count Catholics and Orthodox as fellow Christians. Evangelicals disagree among themselves.",
        )
    } else if x.christian() {
        Outcome::odds(
            0.05,
            0.95,
            0.0,
            "A different Jesus (2 Corinthians 11:4): not God incarnate, or one god among many. Not the gospel, and so not saved.",
        )
    } else {
        Outcome::hell(
            0.0,
            "'No one comes to the Father except through me' (John 14:6). No confession of Christ, no salvation; a decent life is not the criterion (Ephesians 2:8-9).",
        )
    }
}

fn verdict_calvinist(x: &Religion) -> Outcome {
    if x.is("calvinist") {
        Outcome::odds(
            0.85,
            0.15,
            0.0,
            "Whether you were elect was settled before the foundation of the world; following the Reformed faith is evidence of it, not a cause. Some who profess are not elect, and only the elect persevere.",
        )
    } else if x.creedal_christian() {
        Outcome::odds(
            0.6,
            0.4,
            0.0,
            "Faith in Christ is the mark of election, and you had it; wrong about how, which the elect are allowed to be. The evidence is weaker than for the Reformed.",
        )
    } else if x.christian() {
        Outcome::odds(
            0.1,
            0.9,
            0.0,
            "A different gospel is strong evidence of reprobation, though God's decree is his own.",
        )
    } else {
        Outcome::hell(
            0.0,
            "The reprobate are passed over, and dying outside faith in Christ is the evidence of having been passed over. The decree was made before you acted, so nothing you did could have changed it.",
        )
    }
}

fn verdict_lds(x: &Religion) -> Outcome {
    if x.is("lds") {
        Outcome::eternal(
            1.0,
            0.0,
            "Baptised, endowed and sealed in the temple: the celestial kingdom, exaltation, your family with you, worlds without end.",
        )
    } else if x.godless() {
        Outcome::eternal(
            0.65,
            -1.0,
            "'Honorable men of the earth, who were blinded by the craftiness of men' (Doctrine and Covenants 76:75) inherit the terrestrial kingdom, and the gospel is preached in the spirit world, where you may accept it and a proxy baptism done for you. Outer darkness is only for those who knew the truth and fought it; the lowest kingdom of glory 'surpasses all understanding' (76:89).",
        )
    } else {
        Outcome::eternal(
            0.7,
            -1.0,
            "The gospel is preached in the spirit world to everyone who did not hear it in the flesh, and a baptism can be performed for you by proxy; accept it there and the celestial kingdom is open, decline it and the terrestrial kingdom is still glory. Outer darkness is only for those who knew the truth and fought it.",
        )
    }
}

fn verdict_jw(x: &Religion) -> Outcome {
    if x.is("jw") {
        Outcome::eternal(
            0.9,
            0.0,
            "Survive Armageddon, or be resurrected after it, to everlasting life on a paradise earth, if you stay faithful through the final test at the end of the thousand years. The 144,000 go to heaven; the rest of the faithful stay here.",
        )
    } else {
        Outcome::eternal(
            0.45,
            0.0,
            "Die at Armageddon, or before it, and very likely be raised in the resurrection of the unrighteous (Acts 24:15) to a thousand-year judgement day on the paradise earth, judged by what you do then, not by what you did before. The second death is destruction, not torment (Ecclesiastes 9:5, Revelation 20:14); those who knowingly rejected the message are not raised at all.",
        )
    }
}

fn verdict_adventist(x: &Religion) -> Outcome {
    if x.is("adventist") {
        Outcome::eternal(
            0.9,
            0.0,
            "Saved by grace through faith, keeping the Sabbath of the commandment; the dead sleep until the resurrection, and the saved live forever.",
        )
    } else if x.christian() {
        Outcome::eternal(
            0.7,
            0.0,
            "Saved by faith in Christ, judged by the light you had; Sunday keeping becomes the mark of the beast only when the issue has been made plain to you at the end. The lost are burned up in the lake of fire and are gone (Malachi 4:1), not tormented.",
        )
    } else if x.godless() {
        Outcome::eternal(
            0.4,
            0.0,
            "Judged by your response to the light you had (Romans 2:14-16); the dead sleep, the saved rise, and the lost cease to exist after the judgement. Having heard the gospel and rejected it counts against you in a way that never hearing it does not.",
        )
    } else {
        Outcome::eternal(
            0.45,
            0.0,
            "Judged by your response to the light you had (Romans 2:14-16); the dead sleep, the saved rise, and the lost are destroyed, not tormented.",
        )
    }
}

fn verdict_sunni(x: &Religion) -> Outcome {
    if x.is("sunni") {
        Outcome::heaven(
            -2.0,
            "Whoever dies with an atom's weight of faith leaves the Fire in the end; the sins are burned off first, and the Prophet intercedes.",
        )
    } else if x.is("shia") {
        Outcome::odds(
            0.85,
            0.15,
            -3.0,
            "A Muslim, astray on the imamate and the Companions; most scholars count the Shia as Muslims with errors, some Salafi scholars do not. Sins burn off, then paradise.",
        )
    } else if x.is("ahmadiyya") {
        Outcome::odds(
            0.2,
            0.8,
            -3.0,
            "Believing in a prophet after Muhammad denies the seal of prophethood (Qur'an 33:40); the consensus of the schools counts the Ahmadiyya outside Islam, and Pakistan wrote that into law in 1974.",
        )
    } else if x.idols || x.theism == Theism::Polytheist {
        Outcome::hell(
            0.0,
            "Shirk, associating partners with God, is the one sin God does not forgive (Qur'an 4:48); the Fire, without end.",
        )
    } else if x.christian() {
        Outcome::hell(
            0.0,
            "Those who say God is one of three have committed shirk (Qur'an 5:73), and having heard of the Prophet and turned away, you are among the disbelievers; 'whoever seeks a religion other than Islam, it will never be accepted' (3:85). Only those the message never reached are excused.",
        )
    } else {
        Outcome::hell(
            0.0,
            "Having heard of the Prophet and turned away, you are among the disbelievers; 'whoever seeks a religion other than Islam, it will never be accepted' (Qur'an 3:85). Only those the message never reached are excused, and the message reached you.",
        )
    }
}

fn verdict_shia(x: &Religion) -> Outcome {
    if x.is("shia") {
        Outcome::heaven(
            -2.0,
            "Faith in God, the Prophet and the twelve Imams; the sins burn off, the house of the Prophet intercedes, and then paradise.",
        )
    } else if x.is("sunni") {
        Outcome::odds(
            0.6,
            0.4,
            -3.0,
            "A Muslim who missed the Imams; the mustad'af, whom the truth never properly reached, are excused, and those who hated the Prophet's house are not. Which of the two you were is what the case turns on.",
        )
    } else if x.is("ahmadiyya") {
        Outcome::odds(
            0.2,
            0.8,
            -3.0,
            "Believing in a prophet after Muhammad denies the seal of prophethood; outside Islam by the consensus of the schools.",
        )
    } else if x.idols || x.theism == Theism::Polytheist {
        Outcome::hell(
            0.0,
            "Shirk, associating partners with God, is the one sin God does not forgive (Qur'an 4:48); the Fire, without end.",
        )
    } else {
        Outcome::hell(
            0.0,
            "Having heard of the Prophet and turned away, you are among the disbelievers (Qur'an 3:85). Only those the message never reached are excused.",
        )
    }
}

fn verdict_orthodox_judaism(x: &Religion) -> Outcome {
    if x.is("orthodox_judaism") {
        Outcome::eternal(
            1.0,
            -0.5,
            "'All Israel has a share in the World to Come' (Sanhedrin 10:1), after at most twelve months of Gehinnom for the ordinary sinner (Shabbat 33b).",
        )
    } else if x.is("reform_judaism") {
        Outcome::eternal(
            0.95,
            -1.0,
            "Still a Jew and still all Israel; breaking Shabbat costs time in Gehinnom, not the share, though Rosh Hashanah 17a is hard on those who deny the Torah.",
        )
    } else if x.christian() {
        Outcome::eternal(
            0.75,
            -1.0,
            "Shituf, worshipping God together with another, is permitted to gentiles according to most authorities (Tosafot on Sanhedrin 63b, the Rema); Maimonides counted Christianity as idolatry outright. Either way the penalty is Gehinnom for twelve months and no share, never eternal torment, and the righteous of all nations have a share (Tosefta Sanhedrin 13:2).",
        )
    } else if x.idols {
        Outcome::eternal(
            0.3,
            -1.0,
            "Idolatry breaks one of the seven Noahide laws, and the share in the World to Come goes to those who keep them (Tosefta Sanhedrin 13:2). The penalty is Gehinnom for twelve months and then nothing, not torment without end; Judaism has no such place.",
        )
    } else if x.godless() {
        Outcome::eternal(
            0.55,
            -1.0,
            "Not an idolater, and just, which is assumed of every column. Maimonides would withhold the share from one who keeps the seven laws by reason alone rather than because God commanded them (Laws of Kings 8:11); the Tosefta would not.",
        )
    } else if x.family == Family::Gnostic {
        Outcome::eternal(
            0.5,
            -1.0,
            "Not an idolater, but you called the God of Israel a lesser being, which the Talmud counts as heresy. Twelve months of Gehinnom at worst, and the righteous of all nations have a share.",
        )
    } else {
        Outcome::eternal(
            0.9,
            -1.0,
            "'The righteous of all nations have a share in the World to Come' (Tosefta Sanhedrin 13:2); the seven Noahide laws ask for justice and no idolatry, not a creed, and Islam in particular Maimonides counted as pure monotheism.",
        )
    }
}

fn verdict_advaita(x: &Religion) -> Outcome {
    if x.is("advaita") {
        Outcome::heaven(
            -5.0,
            "Knowledge of the self as Brahman ends the round of births, in this life or in a later one; until then samsara, and a decent person is reborn as a human or better.",
        )
    } else if x.family == Family::Hindu {
        Outcome::heaven(
            -10.0,
            "Devotion purifies the mind and prepares it for knowledge; a few more births, then the same realisation.",
        )
    } else if x.dharmic() {
        Outcome::heaven(
            -25.0,
            "You practised restraint and meditation and denied the self that is Brahman; close, and a few more births to see it. Every atman is Brahman whatever it believed.",
        )
    } else if x.godless() {
        Outcome::heaven(
            -40.0,
            "Every atman is Brahman whatever it believed, and you will keep being reborn until you see it; a materialist needs more births than most, but no soul is excluded.",
        )
    } else {
        Outcome::heaven(
            -30.0,
            "Every atman is Brahman whatever it believed, and the other faiths are stages on the same path, in the Advaitin's reading; you keep being reborn until you see it, and your karma decides the births in between.",
        )
    }
}

fn verdict_vaishnava(x: &Religion) -> Outcome {
    if x.is("vaishnava") {
        Outcome::heaven(
            0.0,
            "Back to Godhead: eternal service to Krishna in Goloka Vrindavana, the moment this body ends.",
        )
    } else if x.is("dvaita") {
        Outcome::eternal(
            0.95,
            -5.0,
            "Madhva's line is in the Gaudiya sampradaya's own list of teachers, and you worshipped Vishnu as supreme; a birth or two more, then his abode.",
        )
    } else if x.is("advaita") {
        Outcome::eternal(
            0.8,
            -10.0,
            "Impersonal liberation is real, and the soul falls back down from it, because it is not the soul's nature to be alone; a few more births to recognise the personal God.",
        )
    } else if x.buddhist() {
        Outcome::eternal(
            0.4,
            -30.0,
            "The Buddha is an avatar of Vishnu who taught atheism to lead the demons astray (Bhagavata Purana 1.3.24), and you followed that teaching. Rebirth until you take up devotional service; nothing eternal below Goloka.",
        )
    } else if x.godless() {
        Outcome::eternal(
            0.4,
            -40.0,
            "Denying the Lord is the demoniac nature of the sixteenth chapter of the Gita, and it leads to lower births; but every soul is eternal, hell is finite, and devotional service is available in any birth.",
        )
    } else if x.vegetarian {
        Outcome::eternal(
            0.5,
            -15.0,
            "Rebirth until you take up devotional service; you did not eat meat, which spares you the hells that are counted by the hairs on the animal.",
        )
    } else {
        Outcome::eternal(
            0.5,
            -30.0,
            "Rebirth until you take up devotional service; a decent life leads to a decent birth, and the hells for meat-eating are long but they end.",
        )
    }
}

fn verdict_dvaita(x: &Religion) -> Outcome {
    if x.is("dvaita") {
        Outcome::heaven(
            -3.0,
            "A soul fit for liberation, most likely, since only such a soul takes to Madhva's teaching; Vaikuntha in the end, with a rank there proportioned to your nature.",
        )
    } else if x.is("vaishnava") {
        Outcome::eternal(
            0.95,
            -5.0,
            "Devotion to Vishnu is the mark of the mukti-yogya soul; a birth or two, then Vaikuntha.",
        )
    } else if x.is("advaita") {
        Outcome::eternal(
            0.15,
            -20.0,
            "Madhva called the Advaitins crypto-Buddhists and Sankara a demon in disguise; teaching that the soul is God is what the tamo-yogya soul does, and its end is Andhatamas, darkness without end. Some Advaitins are merely nitya-samsarins and wander forever.",
        )
    } else if x.dharmic() || x.godless() {
        Outcome::eternal(
            0.1,
            -30.0,
            "Denying the Lord, or the Vedas, is the tamo-yogya soul's trait, and its end is Andhatamas without end; a soul of the middle kind wanders forever instead. Which kind a soul is was fixed before it acted, and its conduct only shows which it was.",
        )
    } else {
        Outcome::eternal(
            0.4,
            -30.0,
            "You worshipped God as supreme and separate, which is the mukti-yogya soul's instinct, under another name; but the Vedas are the pramana, and a soul that never accepts them may be one of the middle kind, wandering without end.",
        )
    }
}

fn verdict_theravada(x: &Religion) -> Outcome {
    if x.is("theravada") {
        Outcome::heaven(
            -5.0,
            "Nibbana, in this life or within a few more; the end of dukkha. Even a stream-enterer has at most seven more births.",
        )
    } else if x.buddhist() {
        Outcome::eternal(
            0.9,
            -8.0,
            "The later sutras add to the canon, but the Eightfold Path is in them, and it is what reaches the end of dukkha.",
        )
    } else if x.dharmic() {
        Outcome::eternal(
            0.6,
            -20.0,
            "Restraint and meditation lead to good rebirths and a better chance of hearing the Dhamma; the view of an eternal self (sassatavada) delays you. 'Only here is there a recluse' (Digha Nikaya 16): no arahants outside the Eightfold Path, but Metteyya will teach it again.",
        )
    } else if x.godless() {
        Outcome::eternal(
            0.4,
            -30.0,
            "Annihilationism (ucchedavada) is a wrong view that leads to bad rebirths, though a decent life still earns a decent one; the round has no discoverable beginning, and until you meet the Dhamma it has no end.",
        )
    } else {
        Outcome::eternal(
            0.5,
            -25.0,
            "Good conduct earns a good rebirth, human or heavenly, and heavens end; praying to a creator is a wrong view, but a mild one. You keep being reborn until you hear the Dhamma, and Metteyya will teach it again.",
        )
    }
}

fn verdict_zen(x: &Religion) -> Outcome {
    if x.is("zen") {
        Outcome::heaven(
            0.0,
            "Every being has Buddha-nature, and the practice you kept is the one that wakes it.",
        )
    } else if x.buddhist() {
        Outcome::heaven(
            -3.0,
            "Every being has Buddha-nature and every path in the Dharma leads to the One Vehicle (Lotus Sutra 2).",
        )
    } else if x.dharmic() {
        Outcome::heaven(
            -15.0,
            "Every being has Buddha-nature and will awaken; the bodhisattvas have vowed not to leave until the last one does. Restraint and meditation put you nearer than most.",
        )
    } else if x.godless() {
        Outcome::heaven(
            -30.0,
            "Every being has Buddha-nature and will awaken; the bodhisattvas have vowed not to leave until the last one does. It will take you many lives.",
        )
    } else {
        Outcome::heaven(
            -25.0,
            "Every being has Buddha-nature and will awaken; the bodhisattvas have vowed not to leave until the last one does. A few more lives.",
        )
    }
}

fn verdict_pure_land(x: &Religion) -> Outcome {
    if x.is("pure_land") {
        Outcome::heaven(
            0.0,
            "Ten recitations of the Name with sincere faith (Amida's eighteenth vow): born in Sukhavati, from where Buddhahood is certain.",
        )
    } else if x.is("nichiren") {
        Outcome::eternal(
            0.7,
            -10.0,
            "You refused the Name and called it the road to hell; the vow excludes only the five grave offences and slander of the Dharma, and you came close to the second. Shandao omitted the exclusion clause entirely when he explained the vow.",
        )
    } else if x.buddhist() {
        Outcome::eternal(
            0.9,
            -5.0,
            "Chan and Tendai recite the Name too, and Amida's vow holds whoever calls; self-power is the harder path in the degenerate age, but the vow holds.",
        )
    } else {
        Outcome::eternal(
            0.9,
            -25.0,
            "All beings are within Amida's vow and none is damned forever; you circle until you hear the Name, and then Sukhavati.",
        )
    }
}

fn verdict_vajrayana(x: &Religion) -> Outcome {
    if x.is("vajrayana") {
        Outcome::heaven(
            -2.0,
            "Recognise the clear light in the bardo and there is no more rebirth; miss it and the practice still improves the next one. Breaking samaya with the guru is the one thing that costs kalpas.",
        )
    } else if x.buddhist() {
        Outcome::heaven(
            -5.0,
            "All sentient beings will attain Buddhahood; the sutra path takes three countless aeons where the tantra takes one life, but it arrives.",
        )
    } else if x.dharmic() {
        Outcome::heaven(
            -15.0,
            "All sentient beings will attain Buddhahood; in the bardo you will not recognise the peaceful and wrathful deities as your own mind, and take another birth, a good one.",
        )
    } else {
        Outcome::heaven(
            -25.0,
            "All sentient beings will attain Buddhahood; in the bardo you will not recognise the peaceful and wrathful deities as your own mind, and take another birth.",
        )
    }
}

fn verdict_nichiren(x: &Religion) -> Outcome {
    if x.is("nichiren") {
        Outcome::heaven(
            0.0,
            "Nam-myoho-renge-kyo: Buddhahood in this body, in this lifetime.",
        )
    } else if x.buddhist() {
        Outcome::heaven(
            -500.0,
            "'Nembutsu leads to the hell of incessant suffering, Zen is the teaching of devils, Shingon is an evil doctrine that ruins the nation, Ritsu is treason.' Slandering the Lotus Sutra by preferring a provisional teaching is worse than never hearing it, and Avici lasts kalpas; but the Lotus Sutra promises Buddhahood to everyone, Devadatta included, and so in the end to you.",
        )
    } else {
        Outcome::heaven(
            -100.0,
            "You never heard the Lotus Sutra, so you never slandered it; ordinary bad rebirths, then someone chants for you, then Buddhahood, which the sutra promises to all.",
        )
    }
}

fn verdict_jain(x: &Religion) -> Outcome {
    if x.is("jain") {
        Outcome::heaven(
            -10.0,
            "Right faith, right knowledge, right conduct; the karmic matter falls away over lifetimes and the soul rises to the Siddhashila. Nobody in this age reaches it in one life.",
        )
    } else if x.sacrifice {
        Outcome::eternal(
            0.5,
            -80.0,
            "Killing animals for gods is the heaviest karma there is; the hells of Jainism last up to thirty-three sagaropamas and end, and the soul can still be liberated after, if it is not one of the abhavya that never can be.",
        )
    } else if x.vegetarian {
        Outcome::eternal(
            0.8,
            -20.0,
            "You did not eat meat, which is most of the karma most people bind; a few more births to shed the rest. Whether a soul is capable of liberation at all is its nature, and yours probably is.",
        )
    } else if x.godless() {
        Outcome::eternal(
            0.6,
            -40.0,
            "Jainism has no creator either, so denying one costs nothing; eating meat does. Long rebirths, some in the hells, all of them finite, and liberation possible after, if the soul is of the kind that can.",
        )
    } else {
        Outcome::eternal(
            0.6,
            -40.0,
            "Eating meat binds heavy karma and prayer does not unbind it; long rebirths, some in the hells that end after ages, and liberation possible after, if the soul is of the kind that can.",
        )
    }
}

fn verdict_sikh(x: &Religion) -> Outcome {
    if x.is("sikh") {
        Outcome::heaven(
            -2.0,
            "The Name, the Guru's grace, and truthful living; union with the One, the round of births over.",
        )
    } else if x.theism == Theism::Monotheist && !x.idols {
        Outcome::eternal(
            0.9,
            -10.0,
            "'Truth is high, but higher still is truthful living' (Japji). The Guru Granth Sahib carries the hymns of Kabir and Sheikh Farid, a Muslim; the Gurus taught that the God of every name is the same One. Some births more, by karma.",
        )
    } else if x.idols {
        Outcome::eternal(
            0.7,
            -15.0,
            "Idols and ritual are empty, the Gurus said, and the One behind them is the same One; a truthful life still counts most. Some births more.",
        )
    } else if x.godless() {
        Outcome::eternal(
            0.6,
            -20.0,
            "The ego that denies the One is the thing that keeps you in the round, and a truthful life is still worth more than a creed; some births more, by karma, until grace.",
        )
    } else {
        Outcome::eternal(
            0.7,
            -15.0,
            "'There is no Hindu and no Muslim,' the first Guru said; truthful living counts most. Some births more, by karma, until grace.",
        )
    }
}

fn verdict_daoism(x: &Religion) -> Outcome {
    if x.is("daoism") {
        Outcome::eternal(
            0.3,
            -4.0,
            "Internal alchemy may make you an immortal, a xian, and few practitioners succeed; the rest go through the courts of the underworld like everyone else, into a new life, with their merit counted.",
        )
    } else if x.family == Family::EastAsian || x.buddhist() {
        Outcome::mortal(
            -4.0,
            "The ten courts of Diyu weigh your deeds, punish in proportion, and send you back into a new body; the merit practices you kept are counted.",
        )
    } else {
        Outcome::mortal(
            -5.0,
            "The ten courts of Diyu weigh your deeds, punish in proportion, and send you back into a new body; creed is not counted.",
        )
    }
}

fn verdict_chinese_folk(x: &Religion) -> Outcome {
    if x.family == Family::EastAsian || x.any(&["zen", "pure_land"]) {
        Outcome::mortal(
            -3.0,
            "Judged in the ten courts, punished for your sins in proportion, fed by your descendants' offerings and the paper money they burn, and reborn.",
        )
    } else {
        Outcome::mortal(
            -6.0,
            "The same ten courts and the same punishments, and nobody burning offerings for you: a hungry ghost until the sentence is served, then rebirth.",
        )
    }
}

fn verdict_zoroastrian(x: &Religion) -> Outcome {
    if x.is("zoroastrian") {
        Outcome::heaven(
            0.0,
            "Good thoughts, good words, good deeds; the Chinvat bridge widens for you, and the House of Song. At the Renovation every soul is purified and hell is closed.",
        )
    } else if x.theism == Theism::Polytheist {
        Outcome::heaven(
            -40.0,
            "Worshipping the daevas is the Lie, and the bridge narrows to a blade; the House of Lies until the Renovation, when the molten metal purifies everyone, the wicked with more pain, and hell is closed.",
        )
    } else {
        Outcome::heaven(
            -2.0,
            "The bridge is judged on thoughts, words and deeds, not on creed, and at the Renovation every soul is purified and united with Ahura Mazda; hell is not forever for anyone.",
        )
    }
}

fn verdict_manichaean(x: &Religion) -> Outcome {
    if x.is("manichaean") {
        Outcome::heaven(
            -5.0,
            "The Elect go to the Realm of Light at death; a Hearer is reborn as one of the Elect and follows.",
        )
    } else if x.buddhist() || x.is("zoroastrian") || x.christian() {
        Outcome::eternal(
            0.8,
            -20.0,
            "Mani named the Buddha, Zoroaster and Jesus as his forerunners, and their followers as his, taught earlier; rebirth until you meet the Religion of Light in its full form.",
        )
    } else if x.godless() {
        Outcome::odds(
            0.5,
            0.3,
            -30.0,
            "Rebirth until you hear the Religion of Light; the souls that cling to matter to the end are sealed into the bolos with the darkness, and denying the Light is a way of clinging.",
        )
    } else {
        Outcome::odds(
            0.6,
            0.2,
            -30.0,
            "Rebirth until you hear the Religion of Light; the souls that cling to darkness to the end are sealed into the bolos with it, and most do not.",
        )
    }
}

fn verdict_mandaean(x: &Religion) -> Outcome {
    if x.is("mandaean") {
        Outcome::eternal(
            0.9,
            -3.0,
            "Baptised in living water, sung through the watch-houses by the masiqta, home to the World of Light.",
        )
    } else if x.abrahamic() {
        Outcome::eternal(
            0.2,
            -30.0,
            "The Mandaean books call Jesus a false messiah, Moses a prophet of Ruha, and the religion of the Prophet a persecution; no baptism in the yardna, no masiqta to carry you, and the watch-houses keep you until their fire has purified you.",
        )
    } else {
        Outcome::eternal(
            0.3,
            -30.0,
            "No baptism in the yardna and no masiqta to carry you; the soul is held in the watch-houses and purified there before it can go on. What the end is for the sons of the world, the books do not say clearly.",
        )
    }
}

fn verdict_gnostic(x: &Religion) -> Outcome {
    if x.family == Family::Gnostic {
        Outcome::heaven(
            0.0,
            "The spark knew itself and climbs past the archons to the Pleroma.",
        )
    } else if x.christian() {
        Outcome::eternal(
            0.4,
            -5.0,
            "Faith without gnosis is the psychic Christian's lot: rest in the Ogdoad with the Demiurge, outside the Pleroma but at peace, in the Valentinian account.",
        )
    } else if x.jewish() || x.muslim() {
        Outcome::eternal(
            0.2,
            -10.0,
            "You worshipped the Demiurge as the true God, which is the error gnosis corrects; a psychic at best, and a hylic if the spark never woke.",
        )
    } else if x.dharmic() {
        Outcome::eternal(
            0.3,
            -10.0,
            "You saw that the world is a prison and the self is not what it seems, which is most of gnosis; the Pleroma has a name you did not learn.",
        )
    } else {
        Outcome::eternal(
            0.1,
            -10.0,
            "Hylic, probably: the spark, if it was there, never woke. Back to matter, or another round in the archons' world.",
        )
    }
}

fn verdict_egyptian(x: &Religion) -> Outcome {
    if x.is("egyptian") {
        Outcome::eternal(
            0.85,
            -1.0,
            "Mummified, buried with the Book of Coming Forth by Day, knowing the names of the gatekeepers and the negative confession; the heart weighs less than the feather and you enter the Field of Reeds forever.",
        )
    } else if x.cremates {
        Outcome::eternal(
            0.15,
            -2.0,
            "The ba must return to a body each night, and you burned yours; even a heart lighter than the feather has no body to return to. A heart that fails the weighing is eaten by Ammit, which ends the person.",
        )
    } else {
        Outcome::eternal(
            0.5,
            -2.0,
            "The heart is weighed whatever you believed, and yours was decent; but you did not know the names of the gatekeepers or the words of the negative confession, and a heart that fails the weighing is eaten by Ammit. That is annihilation rather than punishment.",
        )
    }
}

fn verdict_mesopotamian(x: &Religion) -> Outcome {
    if x.is("mesopotamian") {
        Outcome::eternal(
            -0.12,
            0.0,
            "The house of dust, where everyone goes; the kispu offerings of your family keep you fed there, which is the only difference between the columns.",
        )
    } else if x.cremates {
        Outcome::eternal(
            -0.25,
            0.0,
            "The house of dust, where everyone goes, kings and slaves; no grave and no offerings makes it worse, and the unburied dead wander.",
        )
    } else {
        Outcome::eternal(
            -0.2,
            0.0,
            "The house of dust, where everyone goes, kings and slaves; you eat clay and drink dust, and nobody pours water for you.",
        )
    }
}

fn verdict_hellenic(x: &Religion) -> Outcome {
    if x.is("hellenic") {
        Outcome::eternal(
            0.4,
            0.0,
            "Initiated at Eleusis: 'blessed is he who has seen these things', and the initiate hopes for Elysium; the rest of us get the asphodel meadows, and Tartarus is for Tantalus.",
        )
    } else if x.godless() {
        Outcome::eternal(
            -0.15,
            0.0,
            "Impiety is what Socrates was tried for, and the judges of the dead are less lenient than Athens; not Tartarus, which is for those who insulted the gods in person, but a worse part of the meadows.",
        )
    } else if x.abrahamic() {
        Outcome::eternal(
            -0.1,
            0.0,
            "You denied the gods their due for a lifetime; not Tartarus, but a worse part of the asphodel meadows, and no initiation.",
        )
    } else {
        Outcome::eternal(
            -0.05,
            0.0,
            "The asphodel meadows: a grey, forgetful continuation, neither punished nor rewarded. Tartarus is for those who insulted the gods personally.",
        )
    }
}

fn verdict_norse(x: &Religion) -> Outcome {
    if x.is("norse") {
        Outcome::mortal(
            15.0,
            "Die fighting and half the slain go to Odin and half to Freyja, to feast until Ragnarök; die in bed and it is Hel, dim but not painful. Either way it ends when the world burns.",
        )
    } else if x.martial {
        Outcome::mortal(
            10.0,
            "Odin takes the brave slain whoever they prayed to, and you were more likely than most to die that way; Valhalla until Ragnarök, and then the end.",
        )
    } else {
        Outcome::mortal(
            -1.0,
            "Hel is dim rather than painful, and it lasts until Ragnarök ends it. Náströnd is for oathbreakers and murderers, and you were neither.",
        )
    }
}

/// Utility as `eternal · ∞ + finite`, comparing infinite part first.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Utility {
    pub eternal: f64,
    pub finite: f64,
}

impl Utility {
    fn better_than(self, other: Utility) -> bool {
        if (self.eternal - other.eternal).abs() > 1e-9 {
            self.eternal > other.eternal
        } else {
            self.finite > other.finite + 1e-9
        }
    }
}

/// Expected utility of following each tradition, under a prior over which one
/// is true and a per-tradition cost of practice. The prior is normalised; a
/// prior that sums to zero puts every column at nothing but its cost.
pub fn expected_utilities(priors: &[f64], costs: &[f64]) -> Vec<Utility> {
    let n = RELIGIONS.len();
    let total: f64 = priors
        .iter()
        .take(n)
        .filter(|p| p.is_finite() && **p > 0.0)
        .sum();

    (0..n)
        .map(|followed| {
            let mut eternal = 0.0;
            let mut finite = -costs.get(followed).copied().unwrap_or(0.0);
            if total > 0.0 {
                for (truth, prior) in RELIGIONS.iter().zip(priors) {
                    if !prior.is_finite() || *prior <= 0.0 {
                        continue;
                    }
                    let weight = prior / total;
                    let outcome = verdict(truth, &RELIGIONS[followed]);
                    eternal += weight * outcome.eternal;
                    finite += weight * outcome.finite;
                }
            }
            Utility { eternal, finite }
        })
        .collect()
}

/// Indices of the traditions, highest expected utility first.
pub fn ranking(utilities: &[Utility]) -> Vec<u32> {
    let mut order: Vec<u32> = (0..utilities.len() as u32).collect();
    order.sort_by(|a, b| {
        let (ua, ub) = (utilities[*a as usize], utilities[*b as usize]);
        if ua.better_than(ub) {
            std::cmp::Ordering::Less
        } else if ub.better_than(ua) {
            std::cmp::Ordering::Greater
        } else {
            a.cmp(b)
        }
    });
    order
}

/// `+∞`, `−0.6∞ − 3`, `+2`, `0`: an outcome or an expectation as text.
pub fn format_utility(eternal: f64, finite: f64) -> String {
    let eternal = if eternal.abs() < 5e-4 { 0.0 } else { eternal };
    let finite = if finite.abs() < 5e-3 { 0.0 } else { finite };

    let mut text = String::new();
    if eternal != 0.0 {
        text.push(if eternal > 0.0 { '+' } else { '\u{2212}' });
        if (eternal.abs() - 1.0).abs() > 5e-4 {
            text.push_str(&trim_number(&format!("{:.3}", eternal.abs())));
        }
        text.push('\u{221e}');
    }
    if finite != 0.0 {
        let sign = if finite > 0.0 { '+' } else { '\u{2212}' };
        if eternal != 0.0 {
            text.push(' ');
            text.push(sign);
            text.push(' ');
        } else {
            text.push(sign);
        }
        text.push_str(&trim_number(&format!("{:.2}", finite.abs())));
    }
    if text.is_empty() {
        text.push('0');
    }
    text
}

fn trim_number(text: &str) -> String {
    let mut trimmed = text.to_string();
    if trimmed.contains('.') {
        while trimmed.ends_with('0') {
            trimmed.pop();
        }
        if trimmed.ends_with('.') {
            trimmed.pop();
        }
    }
    trimmed
}

#[wasm_bindgen]
pub fn wager_count() -> usize {
    RELIGIONS.len()
}

#[wasm_bindgen]
pub fn wager_id(index: usize) -> String {
    RELIGIONS[index].id.to_string()
}

#[wasm_bindgen]
pub fn wager_name(index: usize) -> String {
    RELIGIONS[index].name.to_string()
}

#[wasm_bindgen]
pub fn wager_family(index: usize) -> String {
    RELIGIONS[index].family.label().to_string()
}

#[wasm_bindgen]
pub fn wager_summary(index: usize) -> String {
    RELIGIONS[index].summary.to_string()
}

#[wasm_bindgen]
pub fn wager_cost(index: usize) -> f64 {
    RELIGIONS[index].cost
}

#[wasm_bindgen]
pub fn wager_adherents(index: usize) -> f64 {
    RELIGIONS[index].adherents
}

#[wasm_bindgen]
pub fn wager_prior(index: usize) -> f64 {
    default_priors()[index]
}

#[wasm_bindgen]
pub fn wager_source_count(index: usize) -> usize {
    sources(RELIGIONS[index].id).len()
}

#[wasm_bindgen]
pub fn wager_source_label(index: usize, source: usize) -> String {
    sources(RELIGIONS[index].id)[source].label.to_string()
}

#[wasm_bindgen]
pub fn wager_source_url(index: usize, source: usize) -> String {
    sources(RELIGIONS[index].id)[source].url.to_string()
}

/// One cell of the table: what `truth` says becomes of a follower of
/// `followed`.
#[wasm_bindgen]
pub struct WagerCell {
    eternal: f64,
    finite: f64,
    note: String,
}

#[wasm_bindgen]
impl WagerCell {
    #[wasm_bindgen(getter)]
    pub fn eternal(&self) -> f64 {
        self.eternal
    }

    #[wasm_bindgen(getter)]
    pub fn finite(&self) -> f64 {
        self.finite
    }

    #[wasm_bindgen(getter)]
    pub fn note(&self) -> String {
        self.note.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn text(&self) -> String {
        format_utility(self.eternal, self.finite)
    }
}

#[wasm_bindgen]
pub fn wager_cell(truth: usize, followed: usize) -> WagerCell {
    let outcome = verdict(&RELIGIONS[truth], &RELIGIONS[followed]);
    WagerCell {
        eternal: outcome.eternal,
        finite: outcome.finite,
        note: outcome.note.to_string(),
    }
}

/// Expected utility per followed tradition, flattened as
/// `[eternal_0, finite_0, eternal_1, finite_1, …]`.
#[wasm_bindgen]
pub fn wager_expected(priors: &[f64], costs: &[f64]) -> Vec<f64> {
    expected_utilities(priors, costs)
        .into_iter()
        .flat_map(|utility| [utility.eternal, utility.finite])
        .collect()
}

/// Indices of the traditions, highest expected utility first, under the given prior and costs.
#[wasm_bindgen]
pub fn wager_ranking(priors: &[f64], costs: &[f64]) -> Vec<u32> {
    ranking(&expected_utilities(priors, costs))
}

#[wasm_bindgen]
pub fn wager_format(eternal: f64, finite: f64) -> String {
    format_utility(eternal, finite)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn index(id: &str) -> usize {
        RELIGIONS
            .iter()
            .position(|religion| religion.id == id)
            .unwrap_or_else(|| panic!("no religion {id}"))
    }

    fn one_hot(id: &str) -> Vec<f64> {
        let mut priors = vec![0.0; RELIGIONS.len()];
        priors[index(id)] = 1.0;
        priors
    }

    fn costs() -> Vec<f64> {
        RELIGIONS.iter().map(|religion| religion.cost).collect()
    }

    #[test]
    fn ids_are_unique_and_every_cell_has_a_verdict() {
        let mut ids: Vec<&str> = RELIGIONS.iter().map(|religion| religion.id).collect();
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(ids.len(), RELIGIONS.len());

        for truth in RELIGIONS {
            for followed in RELIGIONS {
                let outcome = verdict(truth, followed);
                assert!(
                    (-1.0..=1.0).contains(&outcome.eternal),
                    "{} on {}: eternal {}",
                    truth.id,
                    followed.id,
                    outcome.eternal
                );
                assert!(outcome.finite.is_finite());
                assert_ne!(
                    outcome.note, "No verdict recorded.",
                    "{} has no verdict",
                    truth.id
                );
            }
        }
    }

    #[test]
    fn a_godless_world_is_decided_by_cost_alone() {
        for followed in RELIGIONS {
            assert_eq!(verdict(&RELIGIONS[index("atheism")], followed).eternal, 0.0);
        }
        let order = ranking(&expected_utilities(&one_hot("atheism"), &costs()));
        assert_eq!(order[0] as usize, index("atheism"));
    }

    #[test]
    fn a_sunni_world_sends_everyone_but_muslims_to_the_fire() {
        let sunni = &RELIGIONS[index("sunni")];
        for followed in RELIGIONS {
            let outcome = verdict(sunni, followed);
            if followed.muslim() {
                assert!(outcome.eternal > -1.0, "{}", followed.id);
            } else {
                assert_eq!(outcome.eternal, -1.0, "{}", followed.id);
            }
        }
        let order = ranking(&expected_utilities(&one_hot("sunni"), &costs()));
        assert_eq!(order[0] as usize, index("sunni"));
    }

    #[test]
    fn universalists_damn_nobody() {
        for id in ["universalist", "ahmadiyya", "zoroastrian", "bahai", "zen"] {
            let truth = &RELIGIONS[index(id)];
            for followed in RELIGIONS {
                assert_eq!(
                    verdict(truth, followed).eternal,
                    1.0,
                    "{id} on {}",
                    followed.id
                );
            }
        }
    }

    #[test]
    fn judaism_has_no_eternal_hell() {
        for id in ["orthodox_judaism", "reform_judaism"] {
            let truth = &RELIGIONS[index(id)];
            for followed in RELIGIONS {
                assert!(
                    verdict(truth, followed).eternal >= 0.0,
                    "{id} on {}",
                    followed.id
                );
            }
        }
    }

    #[test]
    fn nichiren_treats_other_buddhists_worse_than_strangers() {
        let nichiren = &RELIGIONS[index("nichiren")];
        let zen = verdict(nichiren, &RELIGIONS[index("zen")]);
        let atheist = verdict(nichiren, &RELIGIONS[index("atheism")]);
        assert!(zen.finite < atheist.finite);
    }

    #[test]
    fn infinity_outranks_any_finite_amount() {
        let a = Utility {
            eternal: 0.01,
            finite: -1000.0,
        };
        let b = Utility {
            eternal: 0.0,
            finite: 1000.0,
        };
        assert!(a.better_than(b));
        assert!(!b.better_than(a));
        let tie = Utility {
            eternal: 0.01,
            finite: -999.0,
        };
        assert!(tie.better_than(a));
    }

    #[test]
    fn an_empty_prior_leaves_only_the_costs() {
        let utilities = expected_utilities(&vec![0.0; RELIGIONS.len()], &costs());
        for (utility, religion) in utilities.iter().zip(RELIGIONS) {
            assert_eq!(utility.eternal, 0.0);
            assert_eq!(utility.finite, -religion.cost);
        }
    }

    #[test]
    fn formats_like_the_post() {
        assert_eq!(format_utility(1.0, 0.0), "+∞");
        assert_eq!(format_utility(-1.0, 0.0), "−∞");
        assert_eq!(format_utility(0.6, -3.0), "+0.6∞ − 3");
        assert_eq!(format_utility(-0.05, 0.0), "−0.05∞");
        assert_eq!(format_utility(0.0, 2.0), "+2");
        assert_eq!(format_utility(0.0, -0.15), "−0.15");
        assert_eq!(format_utility(0.0, 0.0), "0");
        assert_eq!(format_utility(0.0001, 0.001), "0");
    }

    /// A verdict nobody can check is an assertion. Every row but naturalism, whose
    /// verdict is that nothing happens, names where its reading came from.
    #[test]
    fn every_row_is_sourced() {
        assert_eq!(SOURCES.len(), RELIGIONS.len());
        for religion in RELIGIONS {
            let sources = sources(religion.id);
            if religion.id == "atheism" {
                assert!(sources.is_empty());
                continue;
            }
            assert!(!sources.is_empty(), "{} has no source", religion.id);
            for source in sources {
                assert!(
                    source.url.starts_with("https://"),
                    "{}: {}",
                    religion.id,
                    source.url
                );
                assert!(!source.label.is_empty());
            }
        }
    }

    /// The table opens on the author's prior rather than a headcount: naturalism
    /// at ninety percent, the rest sharing ten.
    #[test]
    fn the_default_prior_is_mostly_naturalism() {
        let priors = default_priors();
        assert_eq!(priors.len(), RELIGIONS.len());
        assert_eq!(priors[index("atheism")], 90.0);

        let total: f64 = priors.iter().sum();
        assert!((total - 100.0).abs() < 1e-9, "{total}");
        for (prior, religion) in priors.iter().zip(RELIGIONS) {
            if religion.id != "atheism" {
                assert!(*prior < 3.0, "{} at {prior}", religion.id);
            }
        }
    }

    /// Pew's 2020 categories are the source for the headcount prior, so each
    /// family's shares have to add up to the number Pew reports.
    #[test]
    fn adherent_shares_match_the_pew_categories() {
        let share =
            |ids: &[&str]| -> f64 { ids.iter().map(|id| RELIGIONS[index(id)].adherents).sum() };

        let unaffiliated = share(&["atheism", "deism", "pantheism"]);
        assert!((unaffiliated - 24.2).abs() < 0.05, "{unaffiliated}");

        let christian: f64 = RELIGIONS
            .iter()
            .filter(|religion| religion.christian())
            .map(|religion| religion.adherents)
            .sum();
        assert!((christian - 28.8).abs() < 0.05, "{christian}");

        let muslim: f64 = RELIGIONS
            .iter()
            .filter(|religion| religion.muslim())
            .map(|religion| religion.adherents)
            .sum();
        assert!((muslim - 25.6).abs() < 0.05, "{muslim}");

        let hindu = share(&["advaita", "vaishnava", "dvaita"]);
        assert!((hindu - 14.9).abs() < 0.05, "{hindu}");

        let buddhist: f64 = RELIGIONS
            .iter()
            .filter(|religion| religion.buddhist())
            .map(|religion| religion.adherents)
            .sum();
        assert!((buddhist - 4.1).abs() < 0.05, "{buddhist}");

        let jewish: f64 = RELIGIONS
            .iter()
            .filter(|religion| religion.jewish())
            .map(|religion| religion.adherents)
            .sum();
        assert!((jewish - 0.2).abs() < 0.05, "{jewish}");

        let total: f64 = RELIGIONS.iter().map(|religion| religion.adherents).sum();
        assert!((total - 100.0).abs() < 0.1, "{total}");
    }

    #[test]
    fn ranking_is_a_permutation() {
        let priors: Vec<f64> = RELIGIONS
            .iter()
            .map(|religion| religion.adherents)
            .collect();
        let order = ranking(&expected_utilities(&priors, &costs()));
        let mut sorted = order.clone();
        sorted.sort_unstable();
        assert_eq!(sorted, (0..RELIGIONS.len() as u32).collect::<Vec<_>>());
    }
}
