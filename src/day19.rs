use std::{
    collections::HashMap,
    ops::{IndexMut, Range},
};

use aoc_runner_derive::aoc;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    combinator::{iterator, map, map_res, recognize, value},
    multi::separated_list1,
    sequence::{delimited, pair, separated_pair, terminated},
    AsChar, IResult,
};

fn parse_integer(input: &str) -> IResult<&str, usize> {
    map_res(
        recognize(take_while1(|ch: char| ch.is_ascii_digit())),
        |out: &str| out.parse::<usize>(),
    )(input)
}

#[derive(Debug, Clone, Copy)]
enum PropertyName {
    X,
    M,
    A,
    S,
}
impl PropertyName {
    fn parse(input: &str) -> IResult<&str, PropertyName> {
        use PropertyName::*;
        alt((
            value(X, tag("x")),
            value(M, tag("m")),
            value(A, tag("a")),
            value(S, tag("s")),
        ))(input)
    }
}

#[derive(Default, Debug, Clone)]
struct Property<T> {
    x: T,
    m: T,
    a: T,
    s: T,
}

impl<T> std::ops::Index<PropertyName> for Property<T> {
    type Output = T;

    fn index(&self, index: PropertyName) -> &Self::Output {
        use PropertyName::*;
        match index {
            X => &self.x,
            M => &self.m,
            A => &self.a,
            S => &self.s,
        }
    }
}
impl<T> IndexMut<PropertyName> for Property<T> {
    fn index_mut(&mut self, index: PropertyName) -> &mut Self::Output {
        use PropertyName::*;
        match index {
            X => &mut self.x,
            M => &mut self.m,
            A => &mut self.a,
            S => &mut self.s,
        }
    }
}

#[derive(Debug, Clone)]
enum Matcher {
    PropLessThan(PropertyName, usize),
    PropGreaterThan(PropertyName, usize),
}
impl Matcher {
    fn matches(&self, part: &Part) -> bool {
        match self {
            Matcher::PropLessThan(p, threshold) => part.props[*p] < *threshold,
            Matcher::PropGreaterThan(p, threshold) => part.props[*p] > *threshold,
        }
    }

    fn prop_name(&self) -> PropertyName {
        match self {
            Matcher::PropLessThan(p, _) => *p,
            Matcher::PropGreaterThan(p, _) => *p,
        }
    }

    fn parse(input: &str) -> IResult<&str, Matcher> {
        alt((
            map(
                separated_pair(PropertyName::parse, tag(">"), parse_integer),
                |(property, threshold)| Matcher::PropGreaterThan(property, threshold),
            ),
            map(
                separated_pair(PropertyName::parse, tag("<"), parse_integer),
                |(property, threshold)| Matcher::PropLessThan(property, threshold),
            ),
        ))(input)
    }
}

#[derive(Debug, Clone)]
enum IntermediateOutcome {
    Accept,
    Reject,
    Redirect(String),
}
impl IntermediateOutcome {
    fn parse(input: &str) -> IResult<&str, IntermediateOutcome> {
        alt((
            value(IntermediateOutcome::Accept, tag("A")),
            value(IntermediateOutcome::Reject, tag("R")),
            map(take_while1(|ch: char| ch.is_alpha()), |redirect: &str| {
                IntermediateOutcome::Redirect(redirect.to_string())
            }),
        ))(input)
    }
}

#[derive(Debug, Clone)]
struct Rule {
    matcher: Option<Matcher>,
    outcome: IntermediateOutcome,
}

impl Rule {
    fn try_apply(&self, part: &Part) -> Option<IntermediateOutcome> {
        if let Some(matcher) = &self.matcher {
            if matcher.matches(part) {
                Some(self.outcome.clone())
            } else {
                None
            }
        } else {
            Some(self.outcome.clone())
        }
    }

    fn apply_ranges(
        &self,
        part_ranges: &[PartRange],
    ) -> Vec<(PartRange, Option<IntermediateOutcome>)> {
        let mut result = Vec::new();

        if let Some(matcher) = &self.matcher {
            for range in part_ranges {
                let prop_range = range.props[matcher.prop_name()].clone();

                match matcher {
                    Matcher::PropLessThan(prop_name, threshold) => {
                        match (prop_range.start < *threshold, prop_range.end < *threshold) {
                            (true, true) => {
                                // range is entirely below the threshold, so we always apply our action
                                result.push((range.clone(), Some(self.outcome.clone())));
                            }
                            (true, false) => {
                                // range starts below the threshold, ends above it
                                let mut p1 = range.clone();
                                p1.props[*prop_name] = prop_range.start..*threshold;
                                result.push((p1, Some(self.outcome.clone())));

                                let mut p2 = range.clone();
                                p2.props[*prop_name] = *threshold..prop_range.end;
                                result.push((p2, None));
                            }
                            (false, true) => unreachable!(), // can't start above and end below
                            (false, false) => {
                                // range is entirely above the threshold, take no action
                                result.push((range.clone(), None));
                            }
                        }
                    }
                    Matcher::PropGreaterThan(prop_name, threshold) => {
                        match (prop_range.start > *threshold, prop_range.end > *threshold) {
                            (true, true) => {
                                // range is entirely above the threshold, so we always apply our action
                                result.push((range.clone(), Some(self.outcome.clone())));
                            }
                            (true, false) => unreachable!(), // can't start above and end below
                            (false, true) => {
                                // range starts below the threshold, ends above it
                                let mut p1 = range.clone();
                                p1.props[*prop_name] = prop_range.start..*threshold + 1;
                                result.push((p1, None));

                                let mut p2 = range.clone();
                                p2.props[*prop_name] = *threshold + 1..prop_range.end;
                                result.push((p2, Some(self.outcome.clone())));
                            }
                            (false, false) => {
                                // range is entirely above the threshold, take no action
                                result.push((range.clone(), None));
                            }
                        }
                    }
                }
            }
        } else {
            for range in part_ranges {
                result.push((range.clone(), Some(self.outcome.clone())));
            }
        }

        result
    }

    fn parse(input: &str) -> IResult<&str, Rule> {
        alt((
            map(
                separated_pair(Matcher::parse, tag(":"), IntermediateOutcome::parse),
                |(matcher, outcome)| Rule {
                    matcher: Some(matcher),
                    outcome,
                },
            ),
            map(IntermediateOutcome::parse, |outcome| Rule {
                matcher: None,
                outcome,
            }),
        ))(input)
    }
}

#[derive(Debug, Clone)]
struct Workflow {
    rules: Vec<Rule>,
}
impl Workflow {
    fn apply(&self, part: &Part) -> IntermediateOutcome {
        for rule in &self.rules {
            if let Some(outcome) = rule.try_apply(part) {
                return outcome;
            }
        }

        panic!("no matching rule in workflow");
    }

    fn apply_ranges(&self, parts: &[PartRange]) -> Vec<(PartRange, IntermediateOutcome)> {
        let mut result = Vec::new();

        // apply the rules in order
        let mut unprocessed = parts.to_vec();
        for rule in &self.rules {
            let mut still_unprocessed = Vec::new();
            for (subrange, outcome) in rule.apply_ranges(&unprocessed) {
                if let Some(outcome) = outcome {
                    result.push((subrange, outcome));
                } else {
                    still_unprocessed.push(subrange);
                }
            }

            unprocessed = still_unprocessed;
        }

        assert!(unprocessed.is_empty());

        result
    }

    fn parse(input: &str) -> IResult<&str, Workflow> {
        map(separated_list1(tag(","), Rule::parse), |rules| Workflow {
            rules,
        })(input)
    }
}

#[derive(Debug, Clone, Copy)]
enum FinalOutcome {
    Accept,
    Reject,
}
#[derive(Debug, Clone)]
struct WorkflowCollection {
    workflows: HashMap<String, Workflow>,
}
impl WorkflowCollection {
    fn apply(&self, part: &Part) -> FinalOutcome {
        let mut current_workflow = "in".to_string();
        loop {
            match self
                .workflows
                .get(&current_workflow)
                .expect("unknown workflow")
                .apply(part)
            {
                IntermediateOutcome::Accept => return FinalOutcome::Accept,
                IntermediateOutcome::Reject => return FinalOutcome::Reject,
                IntermediateOutcome::Redirect(next_workflow) => {
                    current_workflow = next_workflow.to_string();
                }
            }
        }
    }

    fn apply_ranges(&self, range: PartRange) -> Vec<(PartRange, FinalOutcome)> {
        let mut result = Vec::new();

        let mut unfinished = vec![(range, IntermediateOutcome::Redirect("in".to_string()))];
        while let Some((next_range, intermediate_outcome)) = unfinished.pop() {
            match intermediate_outcome {
                IntermediateOutcome::Accept => result.push((next_range, FinalOutcome::Accept)),
                IntermediateOutcome::Reject => result.push((next_range, FinalOutcome::Reject)),
                IntermediateOutcome::Redirect(next_workflow) => {
                    let next_workflow = self
                        .workflows
                        .get(&next_workflow)
                        .expect("unknown workflow");
                    unfinished.append(&mut next_workflow.apply_ranges(&[next_range]));
                }
            }
        }

        result
    }

    fn parse(input: &str) -> IResult<&str, WorkflowCollection> {
        let mut it = iterator(
            input,
            terminated(
                pair(
                    take_while1(|ch: char| ch.is_alpha()),
                    delimited(tag("{"), Workflow::parse, tag("}")),
                ),
                tag("\n"),
            ),
        );

        let workflows = it
            .map(|(name, workflow)| (name.to_string(), workflow))
            .collect();
        it.finish()
            .map(move |(input, _)| (input, WorkflowCollection { workflows }))
    }
}

#[derive(Debug, Clone)]
struct PartRange {
    props: Property<Range<usize>>,
}
impl Default for PartRange {
    fn default() -> Self {
        Self {
            props: Property {
                x: 1..4001,
                m: 1..4001,
                a: 1..4001,
                s: 1..4001,
            },
        }
    }
}
impl PartRange {
    fn count_num_combos(&self) -> usize {
        use PropertyName::*;
        [X, M, A, S]
            .into_iter()
            .map(|p| self.props[p].end - self.props[p].start)
            .product()
    }
}

#[derive(Default, Debug, Clone)]
struct Part {
    props: Property<usize>,
}
impl Part {
    fn score(&self, outcome: FinalOutcome) -> usize {
        match outcome {
            FinalOutcome::Accept => self.props.x + self.props.m + self.props.a + self.props.s,
            FinalOutcome::Reject => 0,
        }
    }

    fn parse(input: &str) -> IResult<&str, Part> {
        delimited(
            tag("{"),
            map(
                separated_list1(
                    tag(","),
                    separated_pair(PropertyName::parse, tag("="), parse_integer),
                ),
                |properties| {
                    let mut part = Part::default();
                    for (prop_name, value) in properties {
                        part.props[prop_name] = value;
                    }
                    part
                },
            ),
            tag("}"),
        )(input)
    }
}

#[aoc(day19, part1)]
fn part1(input: &str) -> usize {
    let (remaining, workflows) = WorkflowCollection::parse(input).unwrap();
    let mut input = remaining.trim();
    let mut score = 0;
    while !input.is_empty() {
        let (remaining, part) = Part::parse(input).unwrap();
        input = remaining.trim();
        score += part.score(workflows.apply(&part));
    }
    score
}

#[aoc(day19, part2)]
fn part2(input: &str) -> usize {
    let (_, workflows) = WorkflowCollection::parse(input).unwrap();
    let starting_ranges = PartRange::default();
    let final_ranges = workflows.apply_ranges(starting_ranges);
    final_ranges
        .iter()
        .filter(|(_, outcome)| matches!(outcome, FinalOutcome::Accept))
        .fold(0, |acc, (range, _)| acc + range.count_num_combos())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}
 
{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}"#;

    const INPUT: &str = include_str!("../input/2023/day19.txt");
    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 19114);
    }

    #[test]
    fn solves_part1() {
        assert_eq!(part1(INPUT), 331208)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 167409079868000);
    }

    #[test]
    fn solves_part2() {
        assert_eq!(part2(INPUT), 121464316215623);
    }
}
