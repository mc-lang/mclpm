#![allow(dead_code)]
#![allow(unused_macros)]
pub struct Color();
impl Color {
    pub const RESET:     &'static str   = "\x1b[0m";
    pub const BOLD:      &'static str   = "\x1b[1m";
    pub const ITALIC:    &'static str   = "\x1b[3m";
    pub const UNDERLINE: &'static str   = "\x1b[4m";
    pub const BLINK:     &'static str   = "\x1b[5m";
    pub const BLINK2:    &'static str   = "\x1b[6m";
    pub const SELECTED:  &'static str   = "\x1b[7m";
    pub const BLACK:     &'static str   = "\x1b[30m";
    pub const RED:       &'static str   = "\x1b[31m";
    pub const GREEN:     &'static str   = "\x1b[32m";
    pub const YELLOW:    &'static str   = "\x1b[33m";
    pub const BLUE:      &'static str   = "\x1b[34m";
    pub const VIOLET:    &'static str   = "\x1b[35m";
    pub const BEIGE:     &'static str   = "\x1b[36m";
    pub const WHITE:     &'static str   = "\x1b[37m";
    pub const BLACKBG:   &'static str   = "\x1b[40m";
    pub const REDBG:     &'static str   = "\x1b[41m";
    pub const GREENBG:   &'static str   = "\x1b[42m";
    pub const YELLOWBG:  &'static str   = "\x1b[43m";
    pub const BLUEBG:    &'static str   = "\x1b[44m";
    pub const VIOLETBG:  &'static str   = "\x1b[45m";
    pub const BEIGEBG:   &'static str   = "\x1b[46m";
    pub const WHITEBG:   &'static str   = "\x1b[47m";
    pub const GREY:      &'static str   = "\x1b[90m";
    pub const RED2:      &'static str   = "\x1b[91m";
    pub const GREEN2:    &'static str   = "\x1b[92m";
    pub const YELLOW2:   &'static str   = "\x1b[93m";
    pub const BLUE2:     &'static str   = "\x1b[94m";
    pub const VIOLET2:   &'static str   = "\x1b[95m";
    pub const BEIGE2:    &'static str   = "\x1b[96m";
    pub const WHITE2:    &'static str   = "\x1b[97m";
}

#[macro_export] macro_rules! reset{     () => {"\x1b[0m"}}
#[macro_export] macro_rules! bold{      () => {"\x1b[1m"}}
#[macro_export] macro_rules! italic{    () => {"\x1b[3m"}}
#[macro_export] macro_rules! underline{ () => {"\x1b[4m"}}
#[macro_export] macro_rules! blink{     () => {"\x1b[5m"}}
#[macro_export] macro_rules! blink2{    () => {"\x1b[6m"}}
#[macro_export] macro_rules! selected{  () => {"\x1b[7m"}}
#[macro_export] macro_rules! black{     () => {"\x1b[30m"}}
#[macro_export] macro_rules! red{       () => {"\x1b[31m"}}
#[macro_export] macro_rules! green{     () => {"\x1b[32m"}}
#[macro_export] macro_rules! yellow{    () => {"\x1b[33m"}}
#[macro_export] macro_rules! blue{      () => {"\x1b[34m"}}
#[macro_export] macro_rules! violet{    () => {"\x1b[35m"}}
#[macro_export] macro_rules! beige{     () => {"\x1b[36m"}}
#[macro_export] macro_rules! white{     () => {"\x1b[37m"}}
#[macro_export] macro_rules! blackbg{   () => {"\x1b[40m"}}
#[macro_export] macro_rules! redbg{     () => {"\x1b[41m"}}
#[macro_export] macro_rules! greenbg{   () => {"\x1b[42m"}}
#[macro_export] macro_rules! yellowbg{  () => {"\x1b[43m"}}
#[macro_export] macro_rules! bluebg{    () => {"\x1b[44m"}}
#[macro_export] macro_rules! violetbg{  () => {"\x1b[45m"}}
#[macro_export] macro_rules! beigebg{   () => {"\x1b[46m"}}
#[macro_export] macro_rules! whitebg{   () => {"\x1b[47m"}}
#[macro_export] macro_rules! grey{      () => {"\x1b[90m"}}
#[macro_export] macro_rules! red2{      () => {"\x1b[91m"}}
#[macro_export] macro_rules! green2{    () => {"\x1b[92m"}}
#[macro_export] macro_rules! yellow2{   () => {"\x1b[93m"}}
#[macro_export] macro_rules! blue2{     () => {"\x1b[94m"}}
#[macro_export] macro_rules! violet2{   () => {"\x1b[95m"}}
#[macro_export] macro_rules! beige2{    () => {"\x1b[96m"}}
#[macro_export] macro_rules! white2{    () => {"\x1b[97m"}}