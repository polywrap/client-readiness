if [ "$1" = "" ]
then
  echo "Usage: $0 <name of client language>"
  exit
fi

cd ./clients/$1
./run.sh $2

cd ../../verify
./run.sh $1 $2
