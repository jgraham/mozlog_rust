extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use serde_json::Value;

#[derive(Serialize, Deserialize, PartialEq, Copy, Clone, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Status {
    Pass,
    Fail,
    Ok,
    Error,
    Timeout,
    Crash,
    Assert,
    Skip,
}

#[derive(Serialize, Deserialize, PartialEq, Copy, Clone, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SubStatus {
    Pass,
    Fail,
    Error,
    Timeout,
    Assert,
    Notrun,
    Skip,
}

#[derive(Serialize, Deserialize, PartialEq, Copy, Clone, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LogLevel {
    Critical,
    Error,
    Warning,
    Info,
    Debug,
}

#[derive(Serialize, Deserialize, PartialEq, Copy, Clone, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LintLevel {
    Error,
    Warning,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case", tag = "action")]
pub enum LogMessage {
    SuiteStart(SuiteStart),
    SuiteEnd(SuiteEnd),
    TestStart(TestStart),
    TestEnd(TestEnd),
    TestStatus(TestStatus),
    ProcessOutput(ProcessOutput),
    Crash(Crash),
    ValgrindError(ValgrindError),
    ProcessStart(ProcessStart),
    ProcessExit(ProcessExit),
    AssertionCount(AssertionCount),
    Log(Log),
    Lint(Lint),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SuiteStart {
    pub time: u64,
    pub thread: String,
    pub pid: u64,
    pub source: String,
    pub component: Option<String>,
    pub run_info: Option<Value>,
    pub version_info: Option<Value>,
    pub device_info: Option<Value>,
    pub extra: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SuiteEnd {
    pub extra: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TestStart {
    pub time: u64,
    pub thread: String,
    pub pid: u64,
    pub source: String,
    pub component: Option<String>,
    pub test: String,
    pub path: Option<String>,
    pub extra: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TestStatus {
    pub time: u64,
    pub thread: String,
    pub pid: u64,
    pub source: String,
    pub component: Option<String>,
    pub test: String,
    pub subtest: String,
    pub status: SubStatus,
    pub expected: Option<SubStatus>,
    pub message: Option<String>,
    pub stack: Option<String>,
    pub extra: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TestEnd {
    pub time: u64,
    pub thread: String,
    pub pid: u64,
    pub source: String,
    pub component: Option<String>,
    pub test: String,
    pub status: Status,
    pub expected: Option<Status>,
    pub message: Option<String>,
    pub stack: Option<String>,
    pub extra: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProcessOutput {
    time: u64,
    thread: String,
    pid: u64,
    source: String,
    component: Option<String>,
    process: String,
    data: String,
    command: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Crash {
    time: u64,
    thread: String,
    pid: u64,
    source: String,
    component: Option<String>,
    process: Option<String>,
    signature: String,
    test: Option<String>,
    minidump_path: Option<String>,
    minidump_extra: Option<String>,
    stackwalk_retcode: Option<i64>,
    stackwalk_stdout: Option<String>,
    stackwalk_stderr: Option<String>,
    stackwalk_errors: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ValgrindError {
    time: u64,
    thread: String,
    pid: u64,
    source: String,
    component: Option<String>,
    primary: Option<String>,
    secondary: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProcessStart {
    time: u64,
    thread: String,
    pid: u64,
    source: String,
    component: Option<String>,
    process: String,
    command: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProcessExit {
    time: u64,
    thread: String,
    pid: u64,
    source: String,
    component: Option<String>,
    process: String,
    exitcode: i64,
    command: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssertionCount {
    time: u64,
    thread: String,
    pid: u64,
    source: String,
    component: Option<String>,
    test: String,
    count: u64,
    min_expected: i64,
    max_expected: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Log {
    pub time: u64,
    pub thread: String,
    pub pid: u64,
    pub source: String,
    pub component: Option<String>,
    pub level: LogLevel,
    pub message: String,
    pub exc_info: Option<Value>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Lint {
    time: u64,
    thread: String,
    pid: u64,
    source: String,
    component: Option<String>,
    path: String,
    lineno: u64,
    column: Option<u64>,
    hint: Option<String>,
    rule: Option<String>,
    lineoffset: Option<(u64, u64)>,
    linter: Option<String>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
