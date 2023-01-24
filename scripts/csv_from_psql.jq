#psql -h 127.0.0.1 -p 5432 -U openlr openlr_db -c "select row_to_json(x) from (select id,meta,fow,frc,flowdir,from_int,to_int,len,st_astext(geom) as geom,flowdir from local.roads where id in (6882819,8548148) limit 10) as x;" -t | jq -r -f /Users/dave/projects/jq/csv_from_psql.jq
"\(.id):\"\(.meta)\":\(.fow):\(.frc):\(.flowdir):\(.from_int):\(.to_int):\(.len | round):\"\(.geom)\""
