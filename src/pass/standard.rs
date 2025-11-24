//! Standard pass definitions.

#[cfg(feature = "schemars")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::pass::{Architecture, Placement};
use crate::{register::ElementId, SerialCircuit};

/// A standard pass.
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "name")]
#[non_exhaustive]
pub enum StandardPass {
    /// A pass that re-bases the circuit to a custom basis.
    RebaseCustom(RebaseCustom),
    /// A convenience custom rebase that targets TK2.
    RebaseCustomViaTK2,
    /// Automatically rebase to a given gate set.
    AutoRebase(AutoRebase),
    /// Custom squash configuration.
    SquashCustom(SquashCustom),
    /// Automatically squash single-qubit gates.
    AutoSquash(AutoSquash),
    /// Commute multi-qubit gates past other operations.
    CommuteThroughMultis,
    /// Decompose arbitrarily controlled gates.
    DecomposeArbitrarilyControlledGates,
    /// Decompose generic boxes using filters.
    DecomposeBoxes(DecomposeBoxes),
    /// Decompose multi-qubit CX boxes.
    DecomposeMultiQubitsCX,
    /// Decompose single-qubit TK1 boxes.
    DecomposeSingleQubitsTK1,
    /// Two-qubit peephole optimiser.
    PeepholeOptimise2Q(PeepholeOptimise2Q),
    /// Rebase to TKET's default gate set.
    RebaseTket,
    /// Rebase to the UFR gate set.
    RebaseUFR,
    /// Remove redundant operations.
    RemoveRedundancies,
    /// Synthesis pass for TK gate set.
    SynthesiseTK,
    /// Synthesis pass for TKET gate set.
    SynthesiseTket,
    /// Synthesis pass targeting OQC hardware.
    SynthesiseOQC,
    /// Squash TK1 gates.
    SquashTK1,
    /// Squash Rz/PhasedX patterns.
    SquashRzPhasedX,
    /// Flatten registers.
    FlattenRegisters,
    /// Delay measurements.
    DelayMeasures(DelayMeasures),
    /// Convert ZZPhase into Rz rotations.
    ZZPhaseToRz,
    /// Remove discarded outputs.
    RemoveDiscarded,
    /// Simplify measured wires.
    SimplifyMeasured,
    /// Remove explicit barrier operations.
    RemoveBarriers,
    /// Remove phase-only operations.
    RemovePhaseOps,
    /// Decompose bridge gadgets.
    DecomposeBridges,
    /// Run the KAK decomposition.
    KAKDecomposition(KakDecomposition),
    /// Squash arbitrary three-qubit unitaries.
    ThreeQubitSquash(ThreeQubitSquash),
    /// Full peephole optimisation.
    FullPeepholeOptimise(FullPeepholeOptimise),
    /// Compose Phase-Polynomial boxes.
    ComposePhasePolyBoxes(ComposePhasePolyBoxes),
    /// Reduce Euler angles for single qubits.
    EulerAngleReduction(EulerAngleReduction),
    /// Standard routing pipeline.
    RoutingPass(RoutingPass),
    /// Custom routing configuration.
    CustomRoutingPass(CustomRoutingPass),
    /// Configure placements explicitly.
    PlacementPass(PlacementPass),
    /// Naive placement pass.
    NaivePlacementPass(NaivePlacementPass),
    /// Rename qubits.
    RenameQubitsPass(RenameQubitsPass),
    /// Clifford simplification.
    CliffordSimp(CliffordSimp),
    /// Decompose swaps into CXs.
    DecomposeSwapsToCXs(DecomposeSwapsToCxs),
    /// Decompose swaps into custom circuits.
    DecomposeSwapsToCircuit(DecomposeSwapsToCircuit),
    /// Optimise phase gadgets.
    OptimisePhaseGadgets(OptimisePhaseGadgets),
    /// Optimise pairwise gadgets.
    OptimisePairwiseGadgets,
    /// Pauli simplification.
    PauliSimp(PauliSynthesisConfig),
    /// Pauli exponentials synthesis.
    PauliExponentials(PauliSynthesisConfig),
    /// Guided Pauli simplification.
    GuidedPauliSimp(PauliSynthesisConfig),
    /// Simplify initial states.
    SimplifyInitial(SimplifyInitial),
    /// Full mapping pipeline.
    FullMappingPass(FullMappingPass),
    /// Default mapping pipeline.
    DefaultMappingPass(DefaultMappingPass),
    /// CX-focused mapping pipeline.
    CXMappingPass(CxMappingPass),
    /// Pauli squashing.
    PauliSquash(PauliSynthesisConfig),
    /// Context simplification.
    ContextSimp(ContextSimp),
    /// Decompose TK2 gates.
    DecomposeTK2(DecomposeTk2),
    /// Pairwise decomposition of CnX.
    CnXPairwiseDecomposition,
    /// Remove implicit permutation annotations.
    RemoveImplicitQubitPermutation,
    /// Normalise TK2 parameters.
    NormaliseTK2,
    /// Round angles to coarse precision.
    RoundAngles(RoundAngles),
    /// Greedy Pauli simplification with heuristics.
    GreedyPauliSimp(GreedyPauliSimp),
    /// RX synthesis from SX.
    RxFromSX,
    /// Flatten and relabel registers.
    FlattenRelabelRegistersPass(FlattenRelabelRegistersPass),
}

/// A pass that re-bases the circuit to a custom basis.
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RebaseCustom {
    /// OpTypes of supported gates.
    pub basis_allowed: Vec<String>,
    /// A circuit implementing a CX gate in a target gate set.
    pub basis_cx_replacement: Box<SerialCircuit>,
    /// A method for generating optimised single-qubit unitary circuits in a target gate set.
    /// This string should be interpreted by Python "dill" into a function.
    pub basis_tk1_replacement: String,
}

/// Automatically rebase to a given gate set.
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct AutoRebase {
    /// OpTypes of supported gates.
    pub basis_allowed: Vec<String>,
    /// Whether swaps can be introduced while rebasing.
    pub allow_swaps: bool,
}

/// Custom squashing configuration.
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct SquashCustom {
    /// OpTypes of supported single-qubit gates.
    pub basis_singleqs: Vec<String>,
    /// Dill-encoded TK1 replacement method.
    pub basis_tk1_replacement: String,
    /// Whether symbolic gates are always squashed.
    pub always_squash_symbols: bool,
}

/// Automatically squash single-qubit gates.
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct AutoSquash {
    /// OpTypes of supported single-qubit gates.
    pub basis_singleqs: Vec<String>,
}

/// Parameters for decomposing boxes.
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DecomposeBoxes {
    /// Operation types excluded from decomposition.
    pub excluded_types: Vec<String>,
    /// Operation groups excluded from decomposition.
    pub excluded_opgroups: Vec<String>,
    /// Operation types explicitly included in decomposition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub included_types: Option<Vec<String>>,
    /// Operation groups explicitly included in decomposition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub included_opgroups: Option<Vec<String>>,
}

/// Configure the two-qubit peephole optimiser.
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PeepholeOptimise2Q {
    /// Whether swaps can be introduced.
    pub allow_swaps: bool,
}

/// KAK decomposition configuration.
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct KakDecomposition {
    /// Fidelity threshold to preserve semantics.
    pub fidelity: f64,
    /// Whether swaps are allowed during optimisation.
    pub allow_swaps: bool,
    /// Target native 2-qubit gate.
    pub target_2qb_gate: TargetTwoQubitGate,
}

/// Three-qubit squash configuration.
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct ThreeQubitSquash {
    /// Whether swaps are allowed during squashing.
    pub allow_swaps: bool,
}

/// Full peephole optimisation configuration.
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct FullPeepholeOptimise {
    /// Whether swaps are allowed.
    pub allow_swaps: bool,
    /// Target native 2-qubit gate.
    pub target_2qb_gate: TargetTwoQubitGate,
}

/// Compose Phase-Polynomial boxes configuration.
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct ComposePhasePolyBoxes {
    /// Minimal number of CX gates per phase.
    pub min_size: u32,
}

/// Euler angle reduction configuration.
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct EulerAngleReduction {
    /// Axis used for the P rotation.
    pub euler_p: RotationAxis,
    /// Axis used for the Q rotation.
    pub euler_q: RotationAxis,
    /// Whether to enforce strict P-Q-P reductions.
    pub euler_strict: bool,
}

/// Routing pass configuration.
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RoutingPass {
    /// Target architecture.
    pub architecture: Architecture,
    /// Routing configuration.
    pub routing_config: RoutingConfig,
}

/// Custom routing pass configuration.
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct CustomRoutingPass {
    /// Target architecture.
    pub architecture: Architecture,
    /// Routing configuration.
    pub routing_config: RoutingConfig,
}

/// Placement pass configuration.
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PlacementPass {
    /// Placement strategy.
    pub placement: Placement,
}

/// Naive placement configuration.
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct NaivePlacementPass {
    /// Target architecture.
    pub architecture: Architecture,
}

/// Renaming map for qubits.
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RenameQubitsPass {
    /// Mapping between source and destination qubits.
    pub qubit_map: Vec<QubitMapping>,
}

/// Clifford simplification configuration.
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct CliffordSimp {
    /// Whether swaps can be introduced.
    pub allow_swaps: bool,
    /// Target native 2-qubit gate.
    pub target_2qb_gate: TargetTwoQubitGate,
}

/// Decompose swaps into CXs.
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DecomposeSwapsToCxs {
    /// Target architecture.
    pub architecture: Architecture,
    /// Whether the architecture edges are directed.
    pub directed: bool,
}

/// Decompose swaps into custom circuits.
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DecomposeSwapsToCircuit {
    /// Replacement circuit for a swap.
    pub swap_replacement: Box<SerialCircuit>,
}

/// Phase gadget optimisation configuration.
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct OptimisePhaseGadgets {
    /// Preferred CX configuration.
    pub cx_config: CxConfig,
}

/// Shared configuration for Pauli synthesis style passes.
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PauliSynthesisConfig {
    /// Strategy for synthesising Pauli gadgets.
    pub pauli_synth_strat: PauliSynthStrategy,
    /// Preferred CX configuration.
    pub cx_config: CxConfig,
}

/// Simplify initial states configuration.
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct SimplifyInitial {
    /// Whether classical information can be used for simplification.
    pub allow_classical: bool,
    /// Whether to annotate all qubits as initialised to zero.
    pub create_all_qubits: bool,
    /// Optional witness circuit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_circuit: Option<Box<SerialCircuit>>,
}

/// Full mapping pass configuration.
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct FullMappingPass {
    /// Target architecture.
    pub architecture: Architecture,
    /// Placement configuration.
    pub placement: Placement,
    /// Routing configuration.
    pub routing_config: RoutingConfig,
}

/// Default mapping pass configuration.
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DefaultMappingPass {
    /// Target architecture.
    pub architecture: Architecture,
    /// Whether to delay measurements.
    pub delay_measures: bool,
}

/// CX-focused mapping configuration.
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct CxMappingPass {
    /// Target architecture.
    pub architecture: Architecture,
    /// Placement configuration.
    pub placement: Placement,
    /// Routing configuration.
    pub routing_config: RoutingConfig,
    /// Whether the architecture is directed.
    pub directed: bool,
    /// Whether to delay measurements.
    pub delay_measures: bool,
}

/// Pauli context simplification configuration.
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct ContextSimp {
    /// Whether classical information can be used for simplification.
    pub allow_classical: bool,
    /// Reference circuit required by the pass.
    pub x_circuit: Box<SerialCircuit>,
}

/// Decompose TK2 configuration.
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DecomposeTk2 {
    /// Optional fidelity hints.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fidelities: Option<DecomposeTk2Fidelities>,
}

/// Round angles configuration.
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RoundAngles {
    /// Level of precision.
    pub n: i64,
    /// Whether only zero rounding is allowed.
    pub only_zeros: bool,
}

/// Greedy Pauli simplification configuration.
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct GreedyPauliSimp {
    /// Cost discount parameter.
    pub discount_rate: f64,
    /// Depth optimisation weight.
    pub depth_weight: f64,
    /// Maximum lookahead.
    pub max_lookahead: f64,
    /// Maximum number of TQE candidates.
    pub max_tqe_candidates: f64,
    /// Seed for randomness.
    pub seed: f64,
    /// Whether ZZPhase gates may be emitted.
    pub allow_zzphase: bool,
    /// Maximum runtime per thread.
    pub thread_timeout: f64,
    /// Whether only reductions are allowed.
    pub only_reduce: bool,
    /// Number of random trials.
    pub trials: f64,
}

/// Delay measures configuration.
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DelayMeasures {
    /// Whether partial delays are allowed.
    pub allow_partial: bool,
}

/// Flatten and relabel registers configuration.
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct FlattenRelabelRegistersPass {
    /// Label to assign to flattened registers.
    pub label: String,
    /// Whether classical registers should also be relabelled.
    pub relabel_classical_registers: bool,
}

/*
 * ------------------------------------------------------------
 * Pass attribute types.
 * ------------------------------------------------------------
 */

/// Mapping between qubits for renaming operations.
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct QubitMapping(pub ElementId, pub ElementId);

/// Serialized routing configuration.
pub type RoutingConfig = Vec<RoutingMethod>;

/// Routing method descriptor.
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RoutingMethod {
    /// Name of the routing method.
    pub name: String,
}

/// Configuration for decomposing TK2 gates.
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DecomposeTk2Fidelities {
    /// Optional CX fidelity.
    #[serde(rename = "CX", skip_serializing_if = "Option::is_none")]
    pub cx: Option<f64>,
    /// Optional ZZMax fidelity.
    #[serde(rename = "ZZMax", skip_serializing_if = "Option::is_none")]
    pub zz_max: Option<f64>,
    /// Optional ZZPhase fidelity.
    #[serde(rename = "ZZPhase", skip_serializing_if = "Option::is_none")]
    pub zz_phase: Option<f64>,
}

/// Rotation axes used during Euler angle reduction.
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
pub enum RotationAxis {
    /// Rotation around the X axis.
    Rx,
    /// Rotation around the Y axis.
    Ry,
    /// Rotation around the Z axis.
    Rz,
}

/// Target native two-qubit gate for optimisation passes.
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
pub enum TargetTwoQubitGate {
    /// Controlled-NOT
    CX,
    /// TK2 gate.
    TK2,
}

/// Preferred CX configuration for gadget construction.
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
pub enum CxConfig {
    /// Snake configuration.
    Snake,
    /// Tree configuration.
    Tree,
    /// Star configuration.
    Star,
}

/// Strategy for synthesising Pauli gadgets.
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
pub enum PauliSynthStrategy {
    /// Synthesise gadgets individually.
    Individual,
    /// Synthesise gadgets pairwise.
    Pairwise,
    /// Synthesise gadgets in commuting sets.
    Sets,
}
