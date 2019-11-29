/// `enum PropertyId` and the numbers used in `PropertyId::parse` are taken from the bacnet-stack
/// project, `bacenum.h` with minor modifications.
///
/// --- original copyright notice from bacenum.h ---
///
/// Copyright (C) 2012 Steve Karg <skarg@users.sourceforge.net>
///
/// Permission is hereby granted, free of charge, to any person obtaining
/// a copy of this software and associated documentation files (the
/// "Software"), to deal in the Software without restriction, including
/// without limitation the rights to use, copy, modify, merge, publish,
/// distribute, sublicense, and/or sell copies of the Software, and to
/// permit persons to whom the Software is furnished to do so, subject to
/// the following conditions:
///
/// The above copyright notice and this permission notice shall be included
/// in all copies or substantial portions of the Software.
///
/// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
/// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
/// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
/// IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
/// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
/// TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
/// SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
///
///   Modifications Copyright (C) 2017 BACnet Interoperability Testing Services, Inc.
///
///   July 1, 2017    BITS    Modifications to this file have been made in compliance
///                           with original licensing.
///
///   This file contains changes made by BACnet Interoperability Testing
///   Services, Inc. These changes are subject to the permissions,
///   warranty terms and limitations above.
///   For more information: info@bac-test.com
///   For access to source code:  info@bac-test.com
///          or      www.github.com/bacnettesting/bacnet-stack
use arrayref::array_ref;

pub enum PropertyId {
    PropAckedTransitions,
    PropAckRequired,
    PropAction,
    PropActionText,
    PropActiveText,
    PropActiveVtSessions,
    PropAlarmValue,
    PropAlarmValues,
    PropAll,
    PropAllWritesSuccessful,
    PropApduSegmentTimeout,
    PropApduTimeout,
    PropApplicationSoftwareVersion,
    PropArchive,
    PropBias,
    PropChangeOfStateCount,
    PropChangeOfStateTime,
    PropNotificationClass,
    PropBlank1,
    PropControlledVariableReference,
    PropControlledVariableUnits,
    PropControlledVariableValue,
    PropCovIncrement,
    PropDateList,
    PropDaylightSavingsStatus,
    PropDeadband,
    PropDerivativeConstant,
    PropDerivativeConstantUnits,
    PropDescription,
    PropDescriptionOfHalt,
    PropDeviceAddressBinding,
    PropDeviceType,
    PropEffectivePeriod,
    PropElapsedActiveTime,
    PropErrorLimit,
    PropEventEnable,
    PropEventState,
    PropEventType,
    PropExceptionSchedule,
    PropFaultValues,
    PropFeedbackValue,
    PropFileAccessMethod,
    PropFileSize,
    PropFileType,
    PropFirmwareRevision,
    PropHighLimit,
    PropInactiveText,
    PropInProcess,
    PropInstanceOf,
    PropIntegralConstant,
    PropIntegralConstantUnits,
    PropIssueConfirmedNotifications,
    PropLimitEnable,
    PropListOfGroupMembers,
    PropListOfObjectPropertyReferences,
    PropListOfSessionKeys,
    PropLocalDate,
    PropLocalTime,
    PropLocation,
    PropLowLimit,
    PropManipulatedVariableReference,
    PropMaximumOutput,
    PropMaxApduLengthAccepted,
    PropMaxInfoFrames,
    PropMaxMaster,
    PropMaxPresValue,
    PropMinimumOffTime,
    PropMinimumOnTime,
    PropMinimumOutput,
    PropMinPresValue,
    PropModelName,
    PropModificationDate,
    PropNotifyType,
    PropNumberOfApduRetries,
    PropNumberOfStates,
    PropObjectIdentifier,
    PropObjectList,
    PropObjectName,
    PropObjectPropertyReference,
    PropObjectType,
    PropOptional,
    PropOutOfService,
    PropOutputUnits,
    PropEventParameters,
    PropPolarity,
    PropPresentValue,
    PropPriority,
    PropPriorityArray,
    PropPriorityForWriting,
    PropProcessIdentifier,
    PropProgramChange,
    PropProgramLocation,
    PropProgramState,
    PropProportionalConstant,
    PropProportionalConstantUnits,
    PropProtocolConformanceClass, /* deleted in version 1 revision 2 */
    PropProtocolObjectTypesSupported,
    PropProtocolServicesSupported,
    PropProtocolVersion,
    PropReadOnly,
    PropReasonForHalt,
    PropRecipient,
    PropRecipientList,
    PropReliability,
    PropRelinquishDefault,
    PropRequired,
    PropResolution,
    PropSegmentationSupported,
    PropSetpoint,
    PropSetpointReference,
    PropStateText,
    PropStatusFlags,
    PropSystemStatus,
    PropTimeDelay,
    PropTimeOfActiveTimeReset,
    PropTimeOfStateCountReset,
    PropTimeSynchronizationRecipients,
    PropUnits,
    PropUpdateInterval,
    PropUtcOffset,
    PropVendorIdentifier,
    PropVendorName,
    PropVtClassesSupported,
    PropWeeklySchedule,
    PropAttemptedSamples,
    PropAverageValue,
    PropBufferSize,
    PropClientCovIncrement,
    PropCovResubscriptionInterval,
    PropCurrentNotifyTime,
    PropEventTimeStamps,
    PropLogBuffer,
    PropLogDeviceObjectProperty,
    PropEnable,
    PropLogInterval,
    PropMaximumValue,
    PropMinimumValue,
    PropNotificationThreshold,
    PropPreviousNotifyTime,
    PropProtocolRevision,
    PropRecordsSinceNotification,
    PropRecordCount,
    PropStartTime,
    PropStopTime,
    PropStopWhenFull,
    PropTotalRecordCount,
    PropValidSamples,
    PropWindowInterval,
    PropWindowSamples,
    PropMaximumValueTimestamp,
    PropMinimumValueTimestamp,
    PropVarianceValue,
    PropActiveCovSubscriptions,
    PropBackupFailureTimeout,
    PropConfigurationFiles,
    PropDatabaseRevision,
    PropDirectReading,
    PropLastRestoreTime,
    PropMaintenanceRequired,
    PropMemberOf,
    PropMode,
    PropOperationExpected,
    PropSetting,
    PropSilenced,
    PropTrackingValue,
    PropZoneMembers,
    PropLifeSafetyAlarmValues,
    PropMaxSegmentsAccepted,
    PropProfileName,
    PropAutoSlaveDiscovery,
    PropManualSlaveAddressBinding,
    PropSlaveAddressBinding,
    PropSlaveProxyEnable,
    PropLastNotifyRecord,
    PropScheduleDefault,
    PropAcceptedModes,
    PropAdjustValue,
    PropCount,
    PropCountBeforeChange,
    PropCountChangeTime,
    PropCovPeriod,
    PropInputReference,
    PropLimitMonitoringInterval,
    PropLoggingObject,
    PropLoggingRecord,
    PropPrescale,
    PropPulseRate,
    PropScale,
    PropScaleFactor,
    PropUpdateTime,
    PropValueBeforeChange,
    PropValueSet,
    PropValueChangeTime,
    PropAlignIntervals,
    PropIntervalOffset,
    PropLastRestartReason,
    PropLoggingType,
    PropRestartNotificationRecipients,
    PropTimeOfDeviceRestart,
    PropTimeSynchronizationInterval,
    PropTrigger,
    PropUtcTimeSynchronizationRecipients,
    PropNodeSubtype,
    PropNodeType,
    PropStructuredObjectList,
    PropSubordinateAnnotations,
    PropSubordinateList,
    PropActualShedLevel,
    PropDutyWindow,
    PropExpectedShedLevel,
    PropFullDutyBaseline,
    PropRequestedShedLevel,
    PropShedDuration,
    PropShedLevelDescriptions,
    PropShedLevels,
    PropStateDescription,
    PropDoorAlarmState,
    PropDoorExtendedPulseTime,
    PropDoorMembers,
    PropDoorOpenTooLongTime,
    PropDoorPulseTime,
    PropDoorStatus,
    PropDoorUnlockDelayTime,
    PropLockStatus,
    PropMaskedAlarmValues,
    PropSecuredStatus,
    PropAbsenteeLimit,
    PropAccessAlarmEvents,
    PropAccessDoors,
    PropAccessEvent,
    PropAccessEventAuthenticationFactor,
    PropAccessEventCredential,
    PropAccessEventTime,
    PropAccessTransactionEvents,
    PropAccompaniment,
    PropAccompanimentTime,
    PropActivationTime,
    PropActiveAuthenticationPolicy,
    PropAssignedAccessRights,
    PropAuthenticationFactors,
    PropAuthenticationPolicyList,
    PropAuthenticationPolicyNames,
    PropAuthenticationStatus,
    PropAuthorizationMode,
    PropBelongsTo,
    PropCredentialDisable,
    PropCredentialStatus,
    PropCredentials,
    PropCredentialsInZone,
    PropDaysRemaining,
    PropEntryPoints,
    PropExitPoints,
    PropExpirationTime,
    PropExtendedTimeEnable,
    PropFailedAttemptEvents,
    PropFailedAttempts,
    PropFailedAttemptsTime,
    PropLastAccessEvent,
    PropLastAccessPoint,
    PropLastCredentialAdded,
    PropLastCredentialAddedTime,
    PropLastCredentialRemoved,
    PropLastCredentialRemovedTime,
    PropLastUseTime,
    PropLockout,
    PropLockoutRelinquishTime,
    PropMasterExemption,
    PropMaxFailedAttempts,
    PropMembers,
    PropMusterPoint,
    PropNegativeAccessRules,
    PropNumberOfAuthenticationPolicies,
    PropOccupancyCount,
    PropOccupancyCountAdjust,
    PropOccupancyCountEnable,
    PropOccupancyExemption,
    PropOccupancyLowerLimit,
    PropOccupancyLowerLimitEnforced,
    PropOccupancyState,
    PropOccupancyUpperLimit,
    PropOccupancyUpperLimitEnforced,
    PropPassbackExemption,
    PropPassbackMode,
    PropPassbackTimeout,
    PropPositiveAccessRules,
    PropReasonForDisable,
    PropSupportedFormats,
    PropSupportedFormatClasses,
    PropThreatAuthority,
    PropThreatLevel,
    PropTraceFlag,
    PropTransactionNotificationClass,
    PropUserExternalIdentifier,
    PropUserInformationReference,
    PropUserName,
    PropUserType,
    PropUsesRemaining,
    PropZoneFrom,
    PropZoneTo,
    PropAccessEventTag,
    PropGlobalIdentifier,
    PropVerificationTime,
    PropBaseDeviceSecurityPolicy,
    PropDistributionKeyRevision,
    PropDoNotHide,
    PropKeySets,
    PropLastKeyServer,
    PropNetworkAccessSecurityPolicies,
    PropPacketReorderTime,
    PropSecurityPduTimeout,
    PropSecurityTimeWindow,
    PropSupportedSecurityAlgorithm,
    PropUpdateKeySetTimeout,
    PropBackupAndRestoreState,
    PropBackupPreparationTime,
    PropRestoreCompletionTime,
    PropRestorePreparationTime,
    PropBitMask,
    PropBitText,
    PropIsUtc,
    PropGroupMembers,
    PropGroupMemberNames,
    PropMemberStatusFlags,
    PropRequestedUpdateInterval,
    PropCovuPeriod,
    PropCovuRecipients,
    PropEventMessageTexts,
    PropEventMessageTextsConfig,
    PropEventDetectionEnable,
    PropEventAlgorithmInhibit,
    PropEventAlgorithmInhibitRef,
    PropTimeDelayNormal,
    PropReliabilityEvaluationInhibit,
    PropFaultParameters,
    PropFaultType,
    PropLocalForwardingOnly,
    PropProcessIdentifierFilter,
    PropSubscribedRecipients,
    PropPortFilter,
    PropAuthorizationExemptions,
    PropAllowGroupDelayInhibit,
    PropChannelNumber,
    PropControlGroups,
    PropExecutionDelay,
    PropLastPriority,
    PropWriteStatus,
    PropPropertyList,
    PropSerialNumber,
    PropBlinkWarnEnable,
    PropDefaultFadeTime,
    PropDefaultRampRate,
    PropDefaultStepIncrement,
    PropEgressTime,
    PropInProgress,
    PropInstantaneousPower,
    PropLightingCommand,
    PropLightingCommandDefaultPriority,
    PropMaxActualValue,
    PropMinActualValue,
    PropPower,
    PropTransition,
    PropEgressActive,
    PropInterfaceValue,
    PropFaultHighLimit,
    PropFaultLowLimit,
    PropLowDiffLimit,
    PropStrikeCount,
    PropTimeOfStrikeCountReset,
    PropDefaultTimeout,
    PropInitialTimeout,
    PropLastStateChange,
    PropStateChangeValues,
    PropTimerRunning,
    PropTimerState,
    PropApduLength,
    PropIpAddress,
    PropIpDefaultGateway,
    PropIpDhcpEnable,
    PropIpDhcpLeaseTime,
    PropIpDhcpLeaseTimeRemaining,
    PropIpDhcpServer,
    PropIpDnsServer,
    PropBacnetIpGlobalAddress,
    PropBacnetIpMode,
    PropBacnetIpMulticastAddress,
    PropBacnetIpNatTraversal,
    PropIpSubnetMask,
    PropBacnetIpUdpPort,
    PropBbmdAcceptFdRegistrations,
    PropBbmdBroadcastDistributionTable,
    PropBbmdForeignDeviceTable,
    PropChangesPending,
    PropCommand,
    PropFdBbmdAddress,
    PropFdSubscriptionLifetime,
    PropLinkSpeed,
    PropLinkSpeeds,
    PropLinkSpeedAutonegotiate,
    PropMacAddress,
    PropNetworkInterfaceName,
    PropNetworkNumber,
    PropNetworkNumberQuality,
    PropNetworkType,
    PropRoutingTable,
    PropVirtualMacAddressTable,
    PropCommandTimeArray,
    PropCurrentCommandPriority,
    PropLastCommandTime,
    PropValueSource,
    PropValueSourceArray,
    PropBacnetIpv6Mode,
    PropIpv6Address,
    PropIpv6PrefixLength,
    PropBacnetIpv6UdpPort,
    PropIpv6DefaultGateway,
    PropBacnetIpv6MulticastAddress,
    PropIpv6DnsServer,
    PropIpv6AutoAddressingEnable,
    PropIpv6DhcpLeaseTime,
    PropIpv6DhcpLeaseTimeRemaining,
    PropIpv6DhcpServer,
    PropIpv6ZoneIndex,
    PropAssignedLandingCalls,
    PropCarAssignedDirection,
    PropCarDoorCommand,
    PropCarDoorStatus,
    PropCarDoorText,
    PropCarDoorZone,
    PropCarDriveStatus,
    PropCarLoad,
    PropCarLoadUnits,
    PropCarMode,
    PropCarMovingDirection,
    PropCarPosition,
    PropElevatorGroup,
    PropEnergyMeter,
    PropEnergyMeterRef,
    PropEscalatorMode,
    PropFaultSignals,
    PropFloorText,
    PropGroupId,
    PropGroupMode,
    PropHigherDeck,
    PropInstallationId,
    PropLandingCalls,
    PropLandingCallControl,
    PropLandingDoorStatus,
    PropLowerDeck,
    PropMachineRoomId,
    PropMakingCarCall,
    PropNextStoppingFloor,
    PropOperationDirection,
    PropPassengerAlarm,
    PropPowerMode,
    PropRegisteredCarCall,
    PropActiveCovMultipleSubscriptions,
    PropProtocolLevel,
    PropReferencePort,
    PropDeployedProfileLocation,
    PropProfileLocation,
    PropTags,
    PropSubordinateNodeTypes,
    PropSubordinateTags,
    PropSubordinateRelationships,
    PropDefaultSubordinateRelationship,
    PropRepresents,
    Reserved,
    Unknown, // some of these are gaps in the reserved values, the rest are proprietary
}

impl PropertyId {
    pub fn parse(b: &[u8]) -> Self {
        // FIXME: parse properly
        match u16::from_be_bytes(*array_ref!(b, 0, 2)) {
            0 => Self::PropAckedTransitions,
            1 => Self::PropAckRequired,
            2 => Self::PropAction,
            3 => Self::PropActionText,
            4 => Self::PropActiveText,
            5 => Self::PropActiveVtSessions,
            6 => Self::PropAlarmValue,
            7 => Self::PropAlarmValues,
            8 => Self::PropAll,
            9 => Self::PropAllWritesSuccessful,
            10 => Self::PropApduSegmentTimeout,
            11 => Self::PropApduTimeout,
            12 => Self::PropApplicationSoftwareVersion,
            13 => Self::PropArchive,
            14 => Self::PropBias,
            15 => Self::PropChangeOfStateCount,
            16 => Self::PropChangeOfStateTime,
            17 => Self::PropNotificationClass,
            18 => Self::PropBlank1,
            19 => Self::PropControlledVariableReference,
            20 => Self::PropControlledVariableUnits,
            21 => Self::PropControlledVariableValue,
            22 => Self::PropCovIncrement,
            23 => Self::PropDateList,
            24 => Self::PropDaylightSavingsStatus,
            25 => Self::PropDeadband,
            26 => Self::PropDerivativeConstant,
            27 => Self::PropDerivativeConstantUnits,
            28 => Self::PropDescription,
            29 => Self::PropDescriptionOfHalt,
            30 => Self::PropDeviceAddressBinding,
            31 => Self::PropDeviceType,
            32 => Self::PropEffectivePeriod,
            33 => Self::PropElapsedActiveTime,
            34 => Self::PropErrorLimit,
            35 => Self::PropEventEnable,
            36 => Self::PropEventState,
            37 => Self::PropEventType,
            38 => Self::PropExceptionSchedule,
            39 => Self::PropFaultValues,
            40 => Self::PropFeedbackValue,
            41 => Self::PropFileAccessMethod,
            42 => Self::PropFileSize,
            43 => Self::PropFileType,
            44 => Self::PropFirmwareRevision,
            45 => Self::PropHighLimit,
            46 => Self::PropInactiveText,
            47 => Self::PropInProcess,
            48 => Self::PropInstanceOf,
            49 => Self::PropIntegralConstant,
            50 => Self::PropIntegralConstantUnits,
            51 => Self::PropIssueConfirmedNotifications,
            52 => Self::PropLimitEnable,
            53 => Self::PropListOfGroupMembers,
            54 => Self::PropListOfObjectPropertyReferences,
            55 => Self::PropListOfSessionKeys,
            56 => Self::PropLocalDate,
            57 => Self::PropLocalTime,
            58 => Self::PropLocation,
            59 => Self::PropLowLimit,
            60 => Self::PropManipulatedVariableReference,
            61 => Self::PropMaximumOutput,
            62 => Self::PropMaxApduLengthAccepted,
            63 => Self::PropMaxInfoFrames,
            64 => Self::PropMaxMaster,
            65 => Self::PropMaxPresValue,
            66 => Self::PropMinimumOffTime,
            67 => Self::PropMinimumOnTime,
            68 => Self::PropMinimumOutput,
            69 => Self::PropMinPresValue,
            70 => Self::PropModelName,
            71 => Self::PropModificationDate,
            72 => Self::PropNotifyType,
            73 => Self::PropNumberOfApduRetries,
            74 => Self::PropNumberOfStates,
            75 => Self::PropObjectIdentifier,
            76 => Self::PropObjectList,
            77 => Self::PropObjectName,
            78 => Self::PropObjectPropertyReference,
            79 => Self::PropObjectType,
            80 => Self::PropOptional,
            81 => Self::PropOutOfService,
            82 => Self::PropOutputUnits,
            83 => Self::PropEventParameters,
            84 => Self::PropPolarity,
            85 => Self::PropPresentValue,
            86 => Self::PropPriority,
            87 => Self::PropPriorityArray,
            88 => Self::PropPriorityForWriting,
            89 => Self::PropProcessIdentifier,
            90 => Self::PropProgramChange,
            91 => Self::PropProgramLocation,
            92 => Self::PropProgramState,
            93 => Self::PropProportionalConstant,
            94 => Self::PropProportionalConstantUnits,
            95 => Self::PropProtocolConformanceClass, /* deleted in version 1 revision 2 */
            96 => Self::PropProtocolObjectTypesSupported,
            97 => Self::PropProtocolServicesSupported,
            98 => Self::PropProtocolVersion,
            99 => Self::PropReadOnly,
            100 => Self::PropReasonForHalt,
            101 => Self::PropRecipient,
            102 => Self::PropRecipientList,
            103 => Self::PropReliability,
            104 => Self::PropRelinquishDefault,
            105 => Self::PropRequired,
            106 => Self::PropResolution,
            107 => Self::PropSegmentationSupported,
            108 => Self::PropSetpoint,
            109 => Self::PropSetpointReference,
            110 => Self::PropStateText,
            111 => Self::PropStatusFlags,
            112 => Self::PropSystemStatus,
            113 => Self::PropTimeDelay,
            114 => Self::PropTimeOfActiveTimeReset,
            115 => Self::PropTimeOfStateCountReset,
            116 => Self::PropTimeSynchronizationRecipients,
            117 => Self::PropUnits,
            118 => Self::PropUpdateInterval,
            119 => Self::PropUtcOffset,
            120 => Self::PropVendorIdentifier,
            121 => Self::PropVendorName,
            122 => Self::PropVtClassesSupported,
            123 => Self::PropWeeklySchedule,
            124 => Self::PropAttemptedSamples,
            125 => Self::PropAverageValue,
            126 => Self::PropBufferSize,
            127 => Self::PropClientCovIncrement,
            128 => Self::PropCovResubscriptionInterval,
            129 => Self::PropCurrentNotifyTime,
            130 => Self::PropEventTimeStamps,
            131 => Self::PropLogBuffer,
            132 => Self::PropLogDeviceObjectProperty,
            133 => Self::PropEnable,
            134 => Self::PropLogInterval,
            135 => Self::PropMaximumValue,
            136 => Self::PropMinimumValue,
            137 => Self::PropNotificationThreshold,
            138 => Self::PropPreviousNotifyTime,
            139 => Self::PropProtocolRevision,
            140 => Self::PropRecordsSinceNotification,
            141 => Self::PropRecordCount,
            142 => Self::PropStartTime,
            143 => Self::PropStopTime,
            144 => Self::PropStopWhenFull,
            145 => Self::PropTotalRecordCount,
            146 => Self::PropValidSamples,
            147 => Self::PropWindowInterval,
            148 => Self::PropWindowSamples,
            149 => Self::PropMaximumValueTimestamp,
            150 => Self::PropMinimumValueTimestamp,
            151 => Self::PropVarianceValue,
            152 => Self::PropActiveCovSubscriptions,
            153 => Self::PropBackupFailureTimeout,
            154 => Self::PropConfigurationFiles,
            155 => Self::PropDatabaseRevision,
            156 => Self::PropDirectReading,
            157 => Self::PropLastRestoreTime,
            158 => Self::PropMaintenanceRequired,
            159 => Self::PropMemberOf,
            160 => Self::PropMode,
            161 => Self::PropOperationExpected,
            162 => Self::PropSetting,
            163 => Self::PropSilenced,
            164 => Self::PropTrackingValue,
            165 => Self::PropZoneMembers,
            166 => Self::PropLifeSafetyAlarmValues,
            167 => Self::PropMaxSegmentsAccepted,
            168 => Self::PropProfileName,
            169 => Self::PropAutoSlaveDiscovery,
            170 => Self::PropManualSlaveAddressBinding,
            171 => Self::PropSlaveAddressBinding,
            172 => Self::PropSlaveProxyEnable,
            173 => Self::PropLastNotifyRecord,
            174 => Self::PropScheduleDefault,
            175 => Self::PropAcceptedModes,
            176 => Self::PropAdjustValue,
            177 => Self::PropCount,
            178 => Self::PropCountBeforeChange,
            179 => Self::PropCountChangeTime,
            180 => Self::PropCovPeriod,
            181 => Self::PropInputReference,
            182 => Self::PropLimitMonitoringInterval,
            183 => Self::PropLoggingObject,
            184 => Self::PropLoggingRecord,
            185 => Self::PropPrescale,
            186 => Self::PropPulseRate,
            187 => Self::PropScale,
            188 => Self::PropScaleFactor,
            189 => Self::PropUpdateTime,
            190 => Self::PropValueBeforeChange,
            191 => Self::PropValueSet,
            192 => Self::PropValueChangeTime,
            193 => Self::PropAlignIntervals,
            195 => Self::PropIntervalOffset,
            196 => Self::PropLastRestartReason,
            197 => Self::PropLoggingType,
            202 => Self::PropRestartNotificationRecipients,
            203 => Self::PropTimeOfDeviceRestart,
            204 => Self::PropTimeSynchronizationInterval,
            205 => Self::PropTrigger,
            206 => Self::PropUtcTimeSynchronizationRecipients,
            207 => Self::PropNodeSubtype,
            208 => Self::PropNodeType,
            209 => Self::PropStructuredObjectList,
            210 => Self::PropSubordinateAnnotations,
            211 => Self::PropSubordinateList,
            212 => Self::PropActualShedLevel,
            213 => Self::PropDutyWindow,
            214 => Self::PropExpectedShedLevel,
            215 => Self::PropFullDutyBaseline,
            218 => Self::PropRequestedShedLevel,
            219 => Self::PropShedDuration,
            220 => Self::PropShedLevelDescriptions,
            221 => Self::PropShedLevels,
            222 => Self::PropStateDescription,
            226 => Self::PropDoorAlarmState,
            227 => Self::PropDoorExtendedPulseTime,
            228 => Self::PropDoorMembers,
            229 => Self::PropDoorOpenTooLongTime,
            230 => Self::PropDoorPulseTime,
            231 => Self::PropDoorStatus,
            232 => Self::PropDoorUnlockDelayTime,
            233 => Self::PropLockStatus,
            234 => Self::PropMaskedAlarmValues,
            235 => Self::PropSecuredStatus,
            244 => Self::PropAbsenteeLimit,
            245 => Self::PropAccessAlarmEvents,
            246 => Self::PropAccessDoors,
            247 => Self::PropAccessEvent,
            248 => Self::PropAccessEventAuthenticationFactor,
            249 => Self::PropAccessEventCredential,
            250 => Self::PropAccessEventTime,
            251 => Self::PropAccessTransactionEvents,
            252 => Self::PropAccompaniment,
            253 => Self::PropAccompanimentTime,
            254 => Self::PropActivationTime,
            255 => Self::PropActiveAuthenticationPolicy,
            256 => Self::PropAssignedAccessRights,
            257 => Self::PropAuthenticationFactors,
            258 => Self::PropAuthenticationPolicyList,
            259 => Self::PropAuthenticationPolicyNames,
            260 => Self::PropAuthenticationStatus,
            261 => Self::PropAuthorizationMode,
            262 => Self::PropBelongsTo,
            263 => Self::PropCredentialDisable,
            264 => Self::PropCredentialStatus,
            265 => Self::PropCredentials,
            266 => Self::PropCredentialsInZone,
            267 => Self::PropDaysRemaining,
            268 => Self::PropEntryPoints,
            269 => Self::PropExitPoints,
            270 => Self::PropExpirationTime,
            271 => Self::PropExtendedTimeEnable,
            272 => Self::PropFailedAttemptEvents,
            273 => Self::PropFailedAttempts,
            274 => Self::PropFailedAttemptsTime,
            275 => Self::PropLastAccessEvent,
            276 => Self::PropLastAccessPoint,
            277 => Self::PropLastCredentialAdded,
            278 => Self::PropLastCredentialAddedTime,
            279 => Self::PropLastCredentialRemoved,
            280 => Self::PropLastCredentialRemovedTime,
            281 => Self::PropLastUseTime,
            282 => Self::PropLockout,
            283 => Self::PropLockoutRelinquishTime,
            284 => Self::PropMasterExemption,
            285 => Self::PropMaxFailedAttempts,
            286 => Self::PropMembers,
            287 => Self::PropMusterPoint,
            288 => Self::PropNegativeAccessRules,
            289 => Self::PropNumberOfAuthenticationPolicies,
            290 => Self::PropOccupancyCount,
            291 => Self::PropOccupancyCountAdjust,
            292 => Self::PropOccupancyCountEnable,
            293 => Self::PropOccupancyExemption,
            294 => Self::PropOccupancyLowerLimit,
            295 => Self::PropOccupancyLowerLimitEnforced,
            296 => Self::PropOccupancyState,
            297 => Self::PropOccupancyUpperLimit,
            298 => Self::PropOccupancyUpperLimitEnforced,
            299 => Self::PropPassbackExemption,
            300 => Self::PropPassbackMode,
            301 => Self::PropPassbackTimeout,
            302 => Self::PropPositiveAccessRules,
            303 => Self::PropReasonForDisable,
            304 => Self::PropSupportedFormats,
            305 => Self::PropSupportedFormatClasses,
            306 => Self::PropThreatAuthority,
            307 => Self::PropThreatLevel,
            308 => Self::PropTraceFlag,
            309 => Self::PropTransactionNotificationClass,
            310 => Self::PropUserExternalIdentifier,
            311 => Self::PropUserInformationReference,
            317 => Self::PropUserName,
            318 => Self::PropUserType,
            319 => Self::PropUsesRemaining,
            320 => Self::PropZoneFrom,
            321 => Self::PropZoneTo,
            322 => Self::PropAccessEventTag,
            323 => Self::PropGlobalIdentifier,
            326 => Self::PropVerificationTime,
            327 => Self::PropBaseDeviceSecurityPolicy,
            328 => Self::PropDistributionKeyRevision,
            329 => Self::PropDoNotHide,
            330 => Self::PropKeySets,
            331 => Self::PropLastKeyServer,
            332 => Self::PropNetworkAccessSecurityPolicies,
            333 => Self::PropPacketReorderTime,
            334 => Self::PropSecurityPduTimeout,
            335 => Self::PropSecurityTimeWindow,
            336 => Self::PropSupportedSecurityAlgorithm,
            337 => Self::PropUpdateKeySetTimeout,
            338 => Self::PropBackupAndRestoreState,
            339 => Self::PropBackupPreparationTime,
            340 => Self::PropRestoreCompletionTime,
            341 => Self::PropRestorePreparationTime,
            342 => Self::PropBitMask,
            343 => Self::PropBitText,
            344 => Self::PropIsUtc,
            345 => Self::PropGroupMembers,
            346 => Self::PropGroupMemberNames,
            347 => Self::PropMemberStatusFlags,
            348 => Self::PropRequestedUpdateInterval,
            349 => Self::PropCovuPeriod,
            350 => Self::PropCovuRecipients,
            351 => Self::PropEventMessageTexts,
            352 => Self::PropEventMessageTextsConfig,
            353 => Self::PropEventDetectionEnable,
            354 => Self::PropEventAlgorithmInhibit,
            355 => Self::PropEventAlgorithmInhibitRef,
            356 => Self::PropTimeDelayNormal,
            357 => Self::PropReliabilityEvaluationInhibit,
            358 => Self::PropFaultParameters,
            359 => Self::PropFaultType,
            360 => Self::PropLocalForwardingOnly,
            361 => Self::PropProcessIdentifierFilter,
            362 => Self::PropSubscribedRecipients,
            363 => Self::PropPortFilter,
            364 => Self::PropAuthorizationExemptions,
            365 => Self::PropAllowGroupDelayInhibit,
            366 => Self::PropChannelNumber,
            367 => Self::PropControlGroups,
            368 => Self::PropExecutionDelay,
            369 => Self::PropLastPriority,
            370 => Self::PropWriteStatus,
            371 => Self::PropPropertyList,
            372 => Self::PropSerialNumber,
            373 => Self::PropBlinkWarnEnable,
            374 => Self::PropDefaultFadeTime,
            375 => Self::PropDefaultRampRate,
            376 => Self::PropDefaultStepIncrement,
            377 => Self::PropEgressTime,
            378 => Self::PropInProgress,
            379 => Self::PropInstantaneousPower,
            380 => Self::PropLightingCommand,
            381 => Self::PropLightingCommandDefaultPriority,
            382 => Self::PropMaxActualValue,
            383 => Self::PropMinActualValue,
            384 => Self::PropPower,
            385 => Self::PropTransition,
            386 => Self::PropEgressActive,
            387 => Self::PropInterfaceValue,
            388 => Self::PropFaultHighLimit,
            389 => Self::PropFaultLowLimit,
            390 => Self::PropLowDiffLimit,
            391 => Self::PropStrikeCount,
            392 => Self::PropTimeOfStrikeCountReset,
            393 => Self::PropDefaultTimeout,
            394 => Self::PropInitialTimeout,
            395 => Self::PropLastStateChange,
            396 => Self::PropStateChangeValues,
            397 => Self::PropTimerRunning,
            398 => Self::PropTimerState,
            399 => Self::PropApduLength,
            400 => Self::PropIpAddress,
            401 => Self::PropIpDefaultGateway,
            402 => Self::PropIpDhcpEnable,
            403 => Self::PropIpDhcpLeaseTime,
            404 => Self::PropIpDhcpLeaseTimeRemaining,
            405 => Self::PropIpDhcpServer,
            406 => Self::PropIpDnsServer,
            407 => Self::PropBacnetIpGlobalAddress,
            408 => Self::PropBacnetIpMode,
            409 => Self::PropBacnetIpMulticastAddress,
            410 => Self::PropBacnetIpNatTraversal,
            411 => Self::PropIpSubnetMask,
            412 => Self::PropBacnetIpUdpPort,
            413 => Self::PropBbmdAcceptFdRegistrations,
            414 => Self::PropBbmdBroadcastDistributionTable,
            415 => Self::PropBbmdForeignDeviceTable,
            416 => Self::PropChangesPending,
            417 => Self::PropCommand,
            418 => Self::PropFdBbmdAddress,
            419 => Self::PropFdSubscriptionLifetime,
            420 => Self::PropLinkSpeed,
            421 => Self::PropLinkSpeeds,
            422 => Self::PropLinkSpeedAutonegotiate,
            423 => Self::PropMacAddress,
            424 => Self::PropNetworkInterfaceName,
            425 => Self::PropNetworkNumber,
            426 => Self::PropNetworkNumberQuality,
            427 => Self::PropNetworkType,
            428 => Self::PropRoutingTable,
            429 => Self::PropVirtualMacAddressTable,
            430 => Self::PropCommandTimeArray,
            431 => Self::PropCurrentCommandPriority,
            432 => Self::PropLastCommandTime,
            433 => Self::PropValueSource,
            434 => Self::PropValueSourceArray,
            435 => Self::PropBacnetIpv6Mode,
            436 => Self::PropIpv6Address,
            437 => Self::PropIpv6PrefixLength,
            438 => Self::PropBacnetIpv6UdpPort,
            439 => Self::PropIpv6DefaultGateway,
            440 => Self::PropBacnetIpv6MulticastAddress,
            441 => Self::PropIpv6DnsServer,
            442 => Self::PropIpv6AutoAddressingEnable,
            443 => Self::PropIpv6DhcpLeaseTime,
            444 => Self::PropIpv6DhcpLeaseTimeRemaining,
            445 => Self::PropIpv6DhcpServer,
            446 => Self::PropIpv6ZoneIndex,
            447 => Self::PropAssignedLandingCalls,
            448 => Self::PropCarAssignedDirection,
            449 => Self::PropCarDoorCommand,
            450 => Self::PropCarDoorStatus,
            451 => Self::PropCarDoorText,
            452 => Self::PropCarDoorZone,
            453 => Self::PropCarDriveStatus,
            454 => Self::PropCarLoad,
            455 => Self::PropCarLoadUnits,
            456 => Self::PropCarMode,
            457 => Self::PropCarMovingDirection,
            458 => Self::PropCarPosition,
            459 => Self::PropElevatorGroup,
            460 => Self::PropEnergyMeter,
            461 => Self::PropEnergyMeterRef,
            462 => Self::PropEscalatorMode,
            463 => Self::PropFaultSignals,
            464 => Self::PropFloorText,
            465 => Self::PropGroupId,
            467 => Self::PropGroupMode,
            468 => Self::PropHigherDeck,
            469 => Self::PropInstallationId,
            470 => Self::PropLandingCalls,
            471 => Self::PropLandingCallControl,
            472 => Self::PropLandingDoorStatus,
            473 => Self::PropLowerDeck,
            474 => Self::PropMachineRoomId,
            475 => Self::PropMakingCarCall,
            476 => Self::PropNextStoppingFloor,
            477 => Self::PropOperationDirection,
            478 => Self::PropPassengerAlarm,
            479 => Self::PropPowerMode,
            480 => Self::PropRegisteredCarCall,
            481 => Self::PropActiveCovMultipleSubscriptions,
            482 => Self::PropProtocolLevel,
            483 => Self::PropReferencePort,
            484 => Self::PropDeployedProfileLocation,
            485 => Self::PropProfileLocation,
            486 => Self::PropTags,
            487 => Self::PropSubordinateNodeTypes,
            488 => Self::PropSubordinateTags,
            489 => Self::PropSubordinateRelationships,
            490 => Self::PropDefaultSubordinateRelationship,
            491 => Self::PropRepresents,
            492..=511 => Self::Reserved,
            _ => Self::Unknown,
        }
    }
}
