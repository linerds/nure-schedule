#!/usr/bin/env sh

group=11415512
auditorium=-3 # DL :)
teacher=2145721

export ALL_PROXY=socks5://127.0.0.1:9050 # muh privacy

fetch_file() { # file-name, endpoint  <*< ><3
  test -e "test-data/$1" && return;
  printf "Fetching '%s'\n" "$1"
  curl "https://sh.mindenit.org/api/$2" > "test-data/$1"
}

fetch_file "health.json"             "health"

fetch_file "auditoriums.json"        "auditoriums"
fetch_file "auditorium-schedule.json" "auditoriums/${auditorium}/schedule"

fetch_file "groups.json"             "groups"
fetch_file "group-schedule.json"     "groups/${group}/schedule"
fetch_file "group-subjects.json"     "groups/${group}/subjects"
fetch_file "group-teachers.json"     "groups/${group}/teachers"

fetch_file "teachers.json"           "teachers"
fetch_file "teacher-schedule.json"   "teachers/${teacher}/schedule"
