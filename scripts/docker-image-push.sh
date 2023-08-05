# $1 = local name
# $2 = server name
docker image tag $1 $2
docker image push $2