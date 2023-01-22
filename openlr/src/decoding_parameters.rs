#[derive(Clone, Debug)]
pub struct DecodingParameters {
    /// Array which specifies the lowest permissible FRC when considering
    ///  an outbound line during the shortest path search.  The index to
    ///  this array is the LFRC from the source LRP, and the array value
    ///  is the lowest permissable road class (i.e. highest numerical FRC
    ///  value) that a candidate can have to be considered
    pub allowed_frc_delta_table: [usize; 8],

    /// Table of scores for the FRC component during candidate selection
    ///  for an LRP.  Values are in the range 0..1.  Rows correspond to
    ///  the LRP's desired FRC, and columns to the candiate line's actual
    ///  FRC.  The corresponding cell will be the line's FRC score, and
    ///  will the multiplied by the FRC weight.
    pub frc_score_table: [[f64; 8]; 8],

    /// Table of scores for the FOW component during candidate selection
    ///  for an LRP.  Values are in the range 0..1.  Rows correspond to
    ///  the LRP's desired FOW, and columns to the candiate line's actual
    ///  FRC.  The corresponding cell will be the line's FOW score, and
    ///  will the multiplied by the FOW weight.
    pub fow_score_table: [[f64; 8]; 8],

    /// Array of scores for the bearing component during candidate selection
    ///  for an LRP. Values are in the range 0..1. The index into the array
    ///  will be the difference between the LRP's bearing segment and candidate
    ///  line's bearing segment. The corresponding cell will be the line's bearing
    ///  score, and will the multiplied by the bearing weight.
    pub bearing_score_table: [f64; 17],

    /// In case an LRP bearing is not a range (binary), but a scalar(xml),
    /// this value will be multiplied by the candidate edge's bearing and
    /// the LRP bearing.
    pub bearing_delta_penalty: f64,
    /**
     * Weights.
     *    (frc_weight + bearing_weight + fow_weight + distance_weight == 1.0)
     *
     */

    /// Weight of the FRC component.  Value is in the range 0..1
    pub frc_weight: f64,

    /// Weight of the FOW component.  Value is in the range 0..1
    pub fow_weight: f64,

    /// Weight of the bearing component.  Value is in the range 0..1
    pub bearing_weight: f64,

    /// Weight of the distance component.  Value is in the range 0..1.
    ///        The score of the weight will be the candidate_distance / search_radius.
    pub distance_weight: f64,

    /// This limits the number of candidate lines to be selected for an LRP.
    ///  Candidates will be ordered by ascending score and this is an upper limit
    ///  of selected candidates.  Setting this value too high will impact seriously
    ///  performance.
    pub max_candidates_per_lrp: usize,

    /// The number of connected path searches to be attempted.  This value can be
    ///  no higher than m_max_candidated_per_lrp * num_lrps.
    pub max_routing_attempts: usize,

    /// Maximum allowable absolute difference in meters between an LRP's DNP and actual
    ///  LRP-LRP path length. If this parameter is set to "x", then a path will be
    ///  accepted iff: abs(DNP - actual) <= x
    pub max_absolute_dnp_variance: u32,

    /// Maximum allowable relative ratio between an LRP's DNP and actual
    ///  LRP-LRP path length. If this parameter is set to "x", then a
    ///  path will be accepted iff: abs(1 - DNP/actual) <= x
    pub max_relative_dnp_variance: f64,

    /// The maximum allowable rating for a candiate for an LRP.  Value in range
    ///   0..1.  0 indicated a perfect candidate and 1, well, not so much.
    pub max_acceptable_rating: f64,

    /// Search radius in meters around an LRP for candidate lines
    pub search_radius: u32,

    /// Threshold in meters from a candidate's start/end to the LRP projection point
    ///  for that projection point to be snapped to the start/end of the candidate line.
    pub absolute_snapping_threshold: u32,

    /// Threshold in relative length from a candidate's start/end to the LRP projection point
    ///  for that projection point to be snapped to the start/end of the candidate line. Value
    ///  is in range 0..1
    pub relative_snapping_threshold: f64,

    /// Distance in meters from the LRP projection point in the direction of the end of the line
    ///  (for LRPs other than the last LRP), or towards the start of the line (for the last LRP)
    ///  from which the line's bearing is to be determined.  HINT: this defaults to 20, and don't
    ///  change it.
    pub bearing_distance: u32,
}

impl DecodingParameters {
    /// Set an element of the array which specifies the lowest permissible FRC when considering
    ///  an outbound line during the shortest path search.  The index to
    ///  this array is the LFRC from the source LRP, and the array value
    ///  is the lowest permissible road class (i.e. highest numerical FRC
    ///  value) that a candidate can have to be considered
    fn set_allowed_frc_delta(&mut self, index: usize, value: usize) {
        assert!(index < 8);
        self.allowed_frc_delta_table[index] = value;
    }

    /// Set an element of the table of scores for the FRC component during candidate selection
    ///  for an LRP.  Values are in the range 0..1.  Rows correspond to
    ///  the LRP's desired FRC, and columns to the candiate line's actual
    ///  FRC.  The corresponding cell will be the line's FRC score, and
    ///  will the multiplied by the FRC weight.
    fn set_frc_score(&mut self, lrp_frc: usize, edge_frc: usize, score: f64) {
        assert!(lrp_frc < 8);
        assert!(edge_frc < 8);
        assert!((0f64..=1f64).contains(&score));
        self.frc_score_table[lrp_frc][edge_frc] = score;
    }

    /// Set an element of the table of scores for the FOW component during candidate selection
    ///  for an LRP.  Values are in the range 0..1.  Rows correspond to
    ///  the LRP's desired FOW, and columns to the candiate line's actual
    ///  FRC.  The corresponding cell will be the line's FOW score, and
    ///  will the multiplied by the FOW weight.
    fn set_fow_score(&mut self, lrp_fow: usize, edge_fow: usize, score: f64) {
        assert!(lrp_fow < 8);
        assert!(edge_fow < 8);
        assert!((0f64..=1f64).contains(&score));
        self.fow_score_table[lrp_fow][edge_fow] = score;
    }

    /// Set an element of the array of scores for the bearing component during candidate selection
    ///  for an LRP. Values are in the range 0..1. The index into the array
    ///  will be the difference between the LRP's bearing segment and candidate
    ///  line's bearing segment. The corresponding cell will be the line's bearing
    ///  score, and will the multiplied by the bearing weight.
    fn set_bearing_delta_table_penalty(&mut self, bearing_delta: usize, penalty: f64) {
        assert!(bearing_delta < 17);
        assert!((0f64..=1f64).contains(&penalty));
        self.bearing_score_table[bearing_delta] = penalty;
    }

    /// Set the bearing delta penalty. In case an LRP bearing is not a range (binary), but a scalar(xml),
    /// this value will be multiplied by the candidate edge's bearing and
    /// the LRP bearing.
    fn set_bearing_delta_penalty(&mut self, penalty: f64) {
        assert!((0.0f64..=1.0f64).contains(&penalty));
        self.bearing_delta_penalty = penalty;
    }

    /// Set the weight of the FRC component.  Value is in the range 0..1
    fn set_frc_weight(&mut self, weight: f64) {
        assert!((0.0f64..=1.0f64).contains(&weight));
        self.frc_weight = weight;
    }

    /// Set the weight of the FOW component.  Value is in the range 0..1
    fn set_fow_weight(&mut self, weight: f64) {
        assert!((0.0f64..=1.0f64).contains(&weight));
        self.fow_weight = weight;
    }

    /// Set the weight of the bearing component.  Value is in the range 0..1
    fn set_bearing_weight(&mut self, weight: f64) {
        assert!((0.0f64..=1.0f64).contains(&weight));
        self.bearing_weight = weight;
    }

    /// Set the weight of the distance component.  Value is in the range 0..1.
    /// The score of the weight will be the candidate_distance / search_radius.
    fn set_distance_weight(&mut self, weight: f64) {
        assert!((0.0f64..=1.0f64).contains(&weight));
        self.distance_weight = weight;
    }

    /// Set the maximum number of candidate lines to be selected for an LRP.
    ///  Candidates will be ordered by ascending score and this is an upper limit
    ///  of selected candidates.  Setting this value too high will impact seriously
    ///  performance.
    fn set_max_candidates_per_lrp(&mut self, count: usize) {
        self.max_candidates_per_lrp = count;
    }

    /// Set the number of connected path searches to be attempted.  This value can be
    ///  no higher than m_max_candidated_per_lrp * num_lrps.
    fn set_max_routing_attempts(&mut self, count: usize) {
        self.max_routing_attempts = count;
    }

    /// Set the maximum allowable absolute difference in meters between an LRP's DNP and actual
    ///  LRP-LRP path length. If this parameter is set to "x", then a path will be
    ///  accepted iff: abs(DNP - actual) <= x
    fn set_max_absolute_dnp_variance(&mut self, count: u32) {
        self.max_absolute_dnp_variance = count;
    }

    /// Set the maximum allowable relative ratio between an LRP's DNP and actual
    ///  LRP-LRP path length. If this parameter is set to "x", then a
    ///  path will be accepted iff: abs(1 - DNP/actual) <= x
    fn set_max_relative_dnp_variance(&mut self, variance: f64) {
        assert!((0.0f64..=1.0f64).contains(&variance));
        self.max_relative_dnp_variance = variance;
    }

    /// Set the  maximum allowable rating for an edge to be considered as a
    /// candidate for an LRP.  Value in range 0..1.  0 indicated a perfect
    /// candidate and 1, well, not so much.
    fn set_max_acceptable_rating(&mut self, value: f64) {
        self.max_acceptable_rating = value;
    }

    /// set the search radius in meters around an LRP for candidate lines
    fn set_search_radius(&mut self, value: u32) {
        self.search_radius = value;
    }

    /// Set the threshold in meters from a candidate's start/end to the LRP projection point
    ///  for that projection point to be snapped to the start/end of the candidate edge instead
    /// of to the interior of the edge.
    fn set_absolute_snapping_threshold(&mut self, value: u32) {
        self.absolute_snapping_threshold = value;
    }

    /// Set the threshold from a candidate's start/end to the LRP projection point
    ///  for that projection point to be snapped to the start/end of the candidate line.
    ///  The distance is measured as a fraction of the total edge length, and
    ///  is in range 0..1
    fn set_relative_snapping_threshold(&mut self, value: f64) {
        assert!((0.0f64..=1.0f64).contains(&value));
        self.relative_snapping_threshold = value;
    }

    /// Set the distance in meters from the LRP projection point in the direction of the end of the line
    ///  (for LRPs other than the last LRP), or towards the start of the line (for the last LRP)
    ///  from which the line's bearing is to be determined.  HINT: this defaults to 20, and don't
    ///  change it.
    fn set_bearing_distance(&mut self, value: u32) {
        self.bearing_distance = value;
    }
}

impl Default for DecodingParameters {
    fn default() -> Self {
        DecodingParameters {
            allowed_frc_delta_table: [2, 3, 4, 5, 6, 7, 7, 7],
            frc_score_table: [
                [0.00, 0.10, 0.20, 0.30, 0.40, 0.50, 0.60, 0.70], // 0
                [0.10, 0.00, 0.10, 0.20, 0.30, 0.40, 0.50, 0.60], // 1
                [0.20, 0.10, 0.00, 0.10, 0.20, 0.30, 0.40, 0.50], // 2
                [0.30, 0.20, 0.10, 0.00, 0.10, 0.20, 0.30, 0.40], // 3
                [0.40, 0.30, 0.20, 0.10, 0.00, 0.10, 0.20, 0.30], // 4
                [0.50, 0.40, 0.30, 0.20, 0.10, 0.00, 0.10, 0.20], // 5
                [0.60, 0.50, 0.40, 0.30, 0.20, 0.10, 0.00, 0.10], // 6
                [0.70, 0.60, 0.50, 0.40, 0.30, 0.20, 0.10, 0.00], // 7
            ],
            fow_score_table: [
                //UND   MOT   MCW   SCW   RAB   TSQ   SLR   OTH
                [0.00, 0.10, 0.20, 0.30, 0.40, 0.50, 0.60, 0.70], // UND
                [0.10, 0.00, 0.10, 0.20, 0.30, 0.40, 0.50, 0.60], // MOT
                [0.20, 0.10, 0.00, 0.10, 0.20, 0.30, 0.40, 0.50], // MCW
                [0.30, 0.20, 0.10, 0.00, 0.10, 0.20, 0.30, 0.40], // SCW
                [0.40, 0.30, 0.20, 0.10, 0.00, 0.10, 0.20, 0.30], // RAB
                [0.50, 0.40, 0.30, 0.20, 0.10, 0.00, 0.10, 0.20], // TSQ
                [0.60, 0.50, 0.40, 0.30, 0.20, 0.10, 0.00, 0.10], // SLR
                [0.70, 0.60, 0.50, 0.40, 0.30, 0.20, 0.10, 0.00], // OTH
            ],
            bearing_score_table: [
                0.00, 0.00, 0.20, 0.40, 0.60, 0.80, 1.00, 1.00, 1.00, 1.00, 1.00, 1.00, 1.00, 1.00,
                1.00, 1.00, 1.00,
            ],
            bearing_delta_penalty: 0.01,
            frc_weight: 0.1,
            fow_weight: 0.2,
            bearing_weight: 0.2,
            distance_weight: 0.5,
            max_candidates_per_lrp: 5,
            max_routing_attempts: 5,
            max_absolute_dnp_variance: 100,
            max_relative_dnp_variance: 0.4,
            max_acceptable_rating: 0.6,
            search_radius: 30,
            absolute_snapping_threshold: 10,
            relative_snapping_threshold: 0.05,
            bearing_distance: 20,
        }
    }
}

impl DecodingParameters {}

#[cfg(test)]
mod tests {
    use crate::decoding_parameters::DecodingParameters;

    #[test]
    fn it_works() {
        let mp: DecodingParameters = DecodingParameters::default();
        assert_eq!(mp.absolute_snapping_threshold, 10);
    }
}
