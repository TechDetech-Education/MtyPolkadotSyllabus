#/bin/bash

host_ip=`hostname -I | awk '{print $1}' | tr -d "[:space:]"`
gateway=`ip route | grep default | awk '{print $3}'`

neighbours=$(nmap -sn "$host_ip/24" \
  | grep -B 2 "Host is up" \
  | grep "Nmap scan report" \
  | awk '{print $NF}' | tr -d '()' \
  | grep -vwE "$host_ip|$gateway")

if [[ -z "$neighbours" ]]; then
  echo -e "\033[0;31mNo neighbours :(\033[0m"
else
  printf "\033[1;32m%-15s %-10s %-40s\033[0m\n" "IP_ADDR" "PORT" "HOSTNAME(s)"
  for ip in $neighbours; do
    ports=$(nmap -F $ip | grep open | awk '{print $1}' | tr '\n' ',')
    [[ -z "$ports" ]] && ports="N/A"
    
    hostname=$(nslookup $ip | grep name | awk '{print $NF}' | sed 's/\.$//' | tr '\n' '\t\t')
    [[ -z "$hostname" ]] && hostname="N/A"
    
    printf "%-15s %-10s %-40s\n" "$ip" "$ports" "$hostname"
  done
fi
