!!!!!!!!!! ----- Messages file for BOPAlgo package --------- !!!!!!!!!
!  Keyword is name of corresponding alert (error or warning) class

! Special key used to check that messages are loaded; see BOPAlgoOptions
.BOPAlgoLOAD_CHECKER
N/A

.BOPAlgoAlertBOPIsNotSet
The type of Boolean Operation is not set

.BOPAlgoAlertBOPNotAllowed
Boolean operation of the given type is not allowed on the given inputs

.BOPAlgoAlertSolidBuilderFailed
Building Fused solid has failed

.BOPAlgoAlertTooFewArguments
There are no enough arguments to perform the operation

.BOPAlgoAlertMultipleArguments
More than one argument is provided

.BOPAlgoAlertNoFiller
The Pave Filler (the intersection tool) has not been created

.BOPAlgoAlertIntersectionFailed
The intersection of the arguments has failed

.BOPAlgoAlertBuilderFailed
Building of the result shape has failed

.BOPAlgoAlertNullFace
The face given to be split is a null shape

.BOPAlgoAlertNullInputShapes
No or null input shapes

.BOPAlgoAlertPostTreatFF
Cannot connect face intersection curves

.BOPAlgoAlertEmptyShape
Some of the arguments are empty shapes

.BOPAlgoAlertSelfInterferingShape
Some of the arguments are self-interfering shapes

.BOPAlgoAlertTooSmallEdge
Some edges are too small and have no valid range

.BOPAlgoAlertNotSplittableEdge
Some edges are very small and have such a small valid range, that they cannot be split

.BOPAlgoAlertBadPositioning
The positioning of the shapes leads to creation of the small edges without valid range

.BOPAlgoAlertShellSplitterFailed
Unable to build loops from the given faces

.BOPAlgoAlertRemovalOfIBForMDimShapes
Removal of internal boundaries among the multi-dimensional shapes is not supported yet

.BOPAlgoAlertRemovalOfIBForSolidsFailed
Removal of internal boundaries among Solids has failed

.BOPAlgoAlertRemovalOfIBForFacesFailed
Removal of internal boundaries among Faces has failed

.BOPAlgoAlertRemovalOfIBForEdgesFailed
Removal of internal boundaries among Edges has failed

.BOPAlgoAlertIntersectionOfPairOfShapesFailed
Intersection of pair of shapes has failed

.BOPAlgoAlertBuildingPCurveFailed
Building 2D curve of edge on face has failed

.BOPAlgoAlertAcquiredSelfIntersection
Some sub-shapes of some of the argument become connected through other shapes and the argument became self-interfered

.BOPAlgoAlertUnsupportedType
Unsupported type of input shape

.BOPAlgoAlertUnableToRemoveTheFeature
Unable to remove the feature

.BOPAlgoAlertNoFacesToRemove
No faces have been found for removal

.BOPAlgoAlertRemoveFeaturesFailed
The Feature Removal algorithm has failed

.BOPAlgoAlertSolidBuilderUnusedFaces
Some of the faces passed to the Solid Builder algorithm have not been classified and not used for solids creation

.BOPAlgoAlertFaceBuilderUnusedEdges
Some of the edges passed to the Face Builder algorithm have not been classified and not used for faces creation

.BOPAlgoAlertUnableToOrientTheShape
Unable to orient the shape correctly

.BOPAlgoAlertUnknownShape
Shape is unknown for operation

.BOPAlgoAlertNoPeriodicityRequired
No periodicity has been requested for the shape

.BOPAlgoAlertUnableToTrim
Unable to trim the shape for making it periodic (BOP Common fails)

.BOPAlgoAlertUnableToMakeIdentical
Unable to make the shape to look identical on opposite sides (Splitter fails)

.BOPAlgoAlertUnableToRepeat
Unable to repeat the shape (Gluer fails)

.BOPAlgoAlertMultiDimensionalArguments
Multi-dimensional arguments

.BOPAlgoAlertUnableToMakePeriodic
Unable to make the shape periodic

.BOPAlgoAlertUnableToGlue
Unable to glue the shapes

.BOPAlgoAlertShapeIsNotPeriodic
The shape is not periodic

.BOPAlgoAlertUnableToMakeClosedEdgeOnFace
Unable to make closed edge on face.
