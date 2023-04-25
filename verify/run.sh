if [ "$1" = "" ]
then
  echo "Usage: $0 <name of client language>"
  exit
fi


yarn start -- ../clients/$1