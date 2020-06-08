pub const BUILD_SERVER: &'static str = "automaton";
pub const BUILD_DOMAIN: &'static str = "d2.com";
pub const BUILD_SERVER_PORT: u32 = 5000;
pub const BUILD_ROUTE: &'static str = "job/Plans/job/BuildDistributionPipeline/build";
// template param 1 = package 2 = tag
pub const BUILD_PACKAGE_ROUTE: &'static str = "job/Packages/job/{}/tags/job/{}/build";
pub const PARAM_CNT: usize = 6;
pub const OS_VAR: &'static str = "DD_OS";
pub const USERNAME: &'static str = "automaton";
pub const PASSWORD: &'static str = "automatonAdmin!";
pub const SVN_ROOT: &'static str = "http://svnmaster/svn/software/";
pub const GIT_ROOT: &'static str = "http://dd-git.d2.com"; 
pub const DEFAULT_PLATFORM: &'static str = "cent7";