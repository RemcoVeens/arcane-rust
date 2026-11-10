#!/usr/bin/env bash
# Prints your XP and mage rank based on wr's progress database.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
DB=exercises/progress.db

total_quests=0; total_projects=0
for d in exercises/*/*/; do
  [ -f "$d/Cargo.toml" ] || continue
  case "$d" in *_project_*) total_projects=$((total_projects+1));; *) total_quests=$((total_quests+1));; esac
done

quests=0; projects=0; current="—"
if [ -f "$DB" ]; then
  quests=$(sqlite3 "$DB" "select count(*) from open_exercises where solved=1 and exercise not like '%project%';")
  projects=$(sqlite3 "$DB" "select count(*) from open_exercises where solved=1 and exercise like '%project%';")
  current=$(sqlite3 "$DB" "select chapter || '/' || exercise from open_exercises where solved=0 order by chapter, exercise limit 1;")
  [ -n "$current" ] || current="(run 'just quest' to open the next one)"
fi

xp=$((quests*10 + projects*50))
max=$((total_quests*10 + total_projects*50))
ranks=(0 "🕯️  Apprentice" 150 "📘 Initiate" 300 "🔮 Adept" 500 "⚡ Conjurer" 750 "🔥 Sorcerer" 990 "🌌 Archmage")
rank="${ranks[1]}"; next=""
for ((i=0; i<${#ranks[@]}; i+=2)); do
  if (( xp >= ranks[i] )); then rank="${ranks[i+1]}"; else next="${ranks[i+1]} at ${ranks[i]} XP"; break; fi
done

width=30
filled=$(( max > 0 ? xp * width / max : 0 ))
bar=""; for ((i=0; i<width; i++)); do if (( i < filled )); then bar+="█"; else bar+="░"; fi; done

V=$'\e[35m'; G=$'\e[33m'; B=$'\e[1m'; R=$'\e[0m'
echo
echo "${V}  ╔══════════════════════════════════════════╗${R}"
echo "${V}  ║${R}${B}          ✦  THE MAGE'S LEDGER  ✦          ${R}${V}║${R}"
echo "${V}  ╚══════════════════════════════════════════╝${R}"
echo
echo "   Rank      ${B}${rank}${R}"
echo "   XP        ${G}${xp}${R} / ${max}"
echo "   Progress  ${V}${bar}${R}"
echo "   Quests    ${quests} / ${total_quests}    Projects  ${projects} / ${total_projects}"
echo "   Current   ${current}"
[ -n "$next" ] && echo "   Next rank ${next}"
echo
