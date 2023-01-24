# C/+zGCZJgyuvBAAh/x8rHw==
psql -h 127.0.0.1 -p 5432 -U openlr openlr_db -c "select row_to_json(x) from (select id,meta,fow,frc,flowdir,from_int,to_int,len,st_astext(geom) as geom,flowdir from local.roads where geom && ST_MakeEnvelope(-0.425997, 53.837287, -0.418680, 53.843260, 4326)) as x;" -t | jq -r -f ./csv_from_psql.jq
