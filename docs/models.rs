/// Subscription
///
/// category            => problem category
///
struct Subscription {
    id: ID,
}

/// CompanyCode
///
/// code                => invite code used to sign up
///
struct CompanyCode {
    id: ID,
    code: String,
}

/// Company
///
/// companies purchase a subscription
///
/// subscription        => subscription 30 days/1 year
///
struct Company {
    id: ID,
    subscription: Subscription,
    codes: Vec<CompanyCode>
}

/// Team
///
/// members             => users in a team
///
struct Team {
    id: ID,
    name: String,
    members: Vec<User>
}

/// User
///
/// invite_code         => code used to signup
///
struct User {
    id: ID,
    name: String,
    invite_code: CompanyCode
}

/// Contest
///
/// participants        => teams in the contest
///
struct Contest {
    id: ID,
    name: String,
    participants: Vec<Team>
}

/// Challenge
///
/// timer               => timer controling duration
/// problems            => a challenge should contain problems
///
struct Challenge {
    id: ID,
    timer: Timer,
    problems: Vec<Problem>
}

/// Timer
///
/// max_duration        => minutes of how long a problem should take
/// start_time          => start time of problem
/// end_time            => end time of problem
/// elaped_time         => elaped time of problem
///
struct Timer {
    id: ID,
    max_duration: i64,
    start_time: Option<DateTime<utc>>,
    end_time: Option<DateTime<utc>>,
    elaped_time: Option<DateTime<utc>>,
}

/// ServiceCategory
///
/// LOAD_BALANCER       => ie. nginx/haproxy
/// WEB_APPLICATION     => ie. frontend
/// DATABASE            => ie. mysql/postgres
///
enum ServiceCategory {
    LOAD_BALANCER,
    WEB_APPLICATION,
    WEB_SERVICE,
    DATABASE,
}

/// Service
///
/// service_category    => service category
/// max_points          => max points possible from service uptime
/// public_facing       => boolean
///
struct Service {
    id: ID,
    service_category: ServiceCategory,
    max_points: i64,
    public_facing: bool,
}

/// ProblemSkill
///
/// CODE_REVIEW         => review code
/// PR_APPROVAL         => approve code
/// MITIGATION          => prevent exploit
/// RECOVERY            => recover from exploit
/// BACKUP              => create backups
/// ACCESS_ROLES        => control access of users
/// CREDENTIALS         => provision/rotate creds
/// SECRETS_MANAGEMENT  => use a secrets manager
/// AUTOMATION          => script and automate manual tasks
/// THREAT_HUNTING      => find the threat
/// WHITELISTING        => whitelist activity
/// BLACKLISTING        => blacklist activity
/// ANOMOLY_DETECTION   => identify what is normal
/// FIREWALL_RULES      => control ingress/egress traffic
///
enum ProblemSkill {
    CODE_REVIEW,
    PULL_REQUEST_APPROVAL,
    UPDATING,
    BACKUP,
    ACCESS_ROLES,
    USER_PROVISIONING,
    CREDENTIAL_ROTATION,
    SECRETS_MANAGEMENT,
    AUTOMATION,
    INDICATORS_OF_COMPROMISE_ARTIFACTS,
    WHITELISTING,
    BLACKLISTING,
    ANOMOLY_DETECTION,
    FIREWALL_RULES,
}

/// ProblemCatgory
///
/// REVIEW              => reviewing code
/// SYSTEM              => configure system
/// PATCH               => update a package/library
/// SCRIPT              => write a script
/// RULE                => create a firewall/ids rule
/// FORENSICS           => find an artifact
///
enum ProblemCategory {
    REVIEW,
    SYSTEM,
    PATCH,
    SCRIPT,
    RULE,
    FORENSICS,
}

/// ProblemDifficulty
///
/// Individual Contributer Level
/// Years in industry + Credentials + Impactful Projects
///
/// IC1 = entry level, 0 years
/// IC2 = junior, 1-3 years
/// IC3 = senior, 3-5 years
/// IC4 = professional, 5+ years
/// IC5 = team leader/captain/manager, 10+ years
///
/// EASY                => IC1 - IC3
/// HARD                => IC4 = IC5
///
enum ProblemDifficulty {
    EASY,
    HARD,
}

/// Common technology categories commonly used in industry
enum TechnologyCategory {
    INTRUSION_PREVENTION_SYSTEM,
    INTRUSION_DETECTION_SYSTEM,
    FIREWALL,
    WEB_APPLICATION_FIREWALL,
    FILE_INTEGRITY_MONITORING,
    DEBUGGER,
    VULNERABILITY_SCANNER,
    STATIC_ANALYSIS,
    DYNAMIC_ANALYSIS,
}

/// Technology
///
/// The technology used to solve this problem must be easy to use
///
/// category            => technology category
/// git_repository      => github repository
///
struct Technology {
    id: ID,
    category: TechnologyCategory,
    git_repository: String,
}

/// Problems should be solvable according to IC level.
/// Must be a real-world common day-to-day problem
/// The technology used to solve this problem must be easy to use
///
/// category            => problem category
/// technology          => technology involved in problem
/// difficulty          => easy or hard
/// solved              => true or false
/// average_time        => average time of all attempts
/// vulnerabilities     => exposed vulnerabilities that can be exploited
/// service             => services that are deployed to host this problem
/// threats             => attackers participating in challenge
/// max_strikes         => amount of attempt before failing problem
/// solution            => ID of solution
///
struct Problem {
    id: ID,
    category: ProblemCategory,
    technology: Technology,
    difficulty: ProblemDifficulty,
    solved: bool,
    average_time: Option<DateTime<utc>>,
    vulnerabilities: Vec<Vulnerability>,
    services: Vec<Service>,
    threats: Vec<Threat>,
    max_strikes: i64,
    solution: Solution,
}

/// SolutionCategory
///
/// FLAG                => string containing answer
/// HASH                => keep the ingrity of the file
/// TEXT                => some words
/// CODE                => file/string containing code to pass a unit test
///
enum SolutionCategory {
    FLAG,
    HASH,
    TEXT,
    CODE,
}

/// Solution
///
/// category            => the category of te solution
/// content             => the answer to the problem
///
struct Solution {
    id: ID,
    category: SolutionCategory,
    content: String,
}

/// VulnerablilityCategory
///
/// category defined by cve.mitre.org
///
/// MISCONFIGURATION    => should be categorized
/// REMOTE_CODE_EXEC    => remote code execution
///
enum VulnerabilityCategory {
    MISCONFIGURATION,
    REMOTE_CODE_EXEC,
}

/// Vulnerablility
///
/// category            => should be categorized
///
struct Vulnerability {
    id: ID,
    category: VulnerabilityCategory,
}

/// ThreatCategory
///
/// HUMAN               => an human player
/// SCRIPT              => an automated metasploit script
///
enum ThreatCategory {
    HUMAN,
    SCRIPT,
}

/// ThreatDifficulty
///
/// SCRIPT_KIDDIE       => copy pasta scripts
/// BOTNET              => botnet domains
/// APT                 => apt indicators
///
enum ThreatDifficulty {
    SCRIPT_KIDDIE,
    BOTNET,
    APT,
}

/// Threat
///
/// category            => should be categorized
/// difficulty          => should be defined
///
struct Threat {
    id: ID,
    category: ThreatCategory,
    difficulty: ThreatDifficulty,
}

/// ArtifactCategory
///
/// FILE                => file_name + contents
/// PAYLOAD             => data sent
/// NETWORK             => established connection traffic
/// DOMAIN              => dns host
///
enum ArtifactCategory {
    FILE,
    PAYLOAD,
    NETWORK,
    DOMAIN,
}

/// Artifact
///
/// category            => different categories to prove a compromise
///
struct Artifact {
    id: ID,
    category: ArtifactCategory,
}

/// Compromise
///
/// artifacts           => all indicators leading to compromise
///
struct Compromise {
    id: ID,
    artifacts: Vec<Artifact>,
}
