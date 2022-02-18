# \TelephonyProvidersEdgeApi

All URIs are relative to *https://api.mypurecloud.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_telephony_providers_edge**](TelephonyProvidersEdgeApi.md#delete_telephony_providers_edge) | **DELETE** /api/v2/telephony/providers/edges/{edgeId} | Delete a edge.
[**delete_telephony_providers_edge_logicalinterface**](TelephonyProvidersEdgeApi.md#delete_telephony_providers_edge_logicalinterface) | **DELETE** /api/v2/telephony/providers/edges/{edgeId}/logicalinterfaces/{interfaceId} | Delete an edge logical interface
[**delete_telephony_providers_edge_softwareupdate**](TelephonyProvidersEdgeApi.md#delete_telephony_providers_edge_softwareupdate) | **DELETE** /api/v2/telephony/providers/edges/{edgeId}/softwareupdate | Cancels any in-progress update for this edge.
[**delete_telephony_providers_edges_certificateauthority**](TelephonyProvidersEdgeApi.md#delete_telephony_providers_edges_certificateauthority) | **DELETE** /api/v2/telephony/providers/edges/certificateauthorities/{certificateId} | Delete a certificate authority.
[**delete_telephony_providers_edges_didpool**](TelephonyProvidersEdgeApi.md#delete_telephony_providers_edges_didpool) | **DELETE** /api/v2/telephony/providers/edges/didpools/{didPoolId} | Delete a DID Pool by ID.
[**delete_telephony_providers_edges_edgegroup**](TelephonyProvidersEdgeApi.md#delete_telephony_providers_edges_edgegroup) | **DELETE** /api/v2/telephony/providers/edges/edgegroups/{edgeGroupId} | Delete an edge group.
[**delete_telephony_providers_edges_extensionpool**](TelephonyProvidersEdgeApi.md#delete_telephony_providers_edges_extensionpool) | **DELETE** /api/v2/telephony/providers/edges/extensionpools/{extensionPoolId} | Delete an extension pool by ID
[**delete_telephony_providers_edges_outboundroute**](TelephonyProvidersEdgeApi.md#delete_telephony_providers_edges_outboundroute) | **DELETE** /api/v2/telephony/providers/edges/outboundroutes/{outboundRouteId} | Delete Outbound Route
[**delete_telephony_providers_edges_phone**](TelephonyProvidersEdgeApi.md#delete_telephony_providers_edges_phone) | **DELETE** /api/v2/telephony/providers/edges/phones/{phoneId} | Delete a Phone by ID
[**delete_telephony_providers_edges_phonebasesetting**](TelephonyProvidersEdgeApi.md#delete_telephony_providers_edges_phonebasesetting) | **DELETE** /api/v2/telephony/providers/edges/phonebasesettings/{phoneBaseId} | Delete a Phone Base Settings by ID
[**delete_telephony_providers_edges_site**](TelephonyProvidersEdgeApi.md#delete_telephony_providers_edges_site) | **DELETE** /api/v2/telephony/providers/edges/sites/{siteId} | Delete a Site by ID
[**delete_telephony_providers_edges_site_outboundroute**](TelephonyProvidersEdgeApi.md#delete_telephony_providers_edges_site_outboundroute) | **DELETE** /api/v2/telephony/providers/edges/sites/{siteId}/outboundroutes/{outboundRouteId} | Delete Outbound Route
[**delete_telephony_providers_edges_trunkbasesetting**](TelephonyProvidersEdgeApi.md#delete_telephony_providers_edges_trunkbasesetting) | **DELETE** /api/v2/telephony/providers/edges/trunkbasesettings/{trunkBaseSettingsId} | Delete a Trunk Base Settings object by ID
[**get_configuration_schemas_edges_vnext**](TelephonyProvidersEdgeApi.md#get_configuration_schemas_edges_vnext) | **GET** /api/v2/configuration/schemas/edges/vnext | Lists available schema categories (Deprecated)
[**get_configuration_schemas_edges_vnext_schema_category**](TelephonyProvidersEdgeApi.md#get_configuration_schemas_edges_vnext_schema_category) | **GET** /api/v2/configuration/schemas/edges/vnext/{schemaCategory} | List schemas of a specific category (Deprecated)
[**get_configuration_schemas_edges_vnext_schema_category_schema_type**](TelephonyProvidersEdgeApi.md#get_configuration_schemas_edges_vnext_schema_category_schema_type) | **GET** /api/v2/configuration/schemas/edges/vnext/{schemaCategory}/{schemaType} | List schemas of a specific category (Deprecated)
[**get_configuration_schemas_edges_vnext_schema_category_schema_type_schema_id**](TelephonyProvidersEdgeApi.md#get_configuration_schemas_edges_vnext_schema_category_schema_type_schema_id) | **GET** /api/v2/configuration/schemas/edges/vnext/{schemaCategory}/{schemaType}/{schemaId} | Get a json schema (Deprecated)
[**get_configuration_schemas_edges_vnext_schema_category_schema_type_schema_id_extension_type_metadata_id**](TelephonyProvidersEdgeApi.md#get_configuration_schemas_edges_vnext_schema_category_schema_type_schema_id_extension_type_metadata_id) | **GET** /api/v2/configuration/schemas/edges/vnext/{schemaCategory}/{schemaType}/{schemaId}/{extensionType}/{metadataId} | Get metadata for a schema (Deprecated)
[**get_telephony_providers_edge**](TelephonyProvidersEdgeApi.md#get_telephony_providers_edge) | **GET** /api/v2/telephony/providers/edges/{edgeId} | Get edge.
[**get_telephony_providers_edge_diagnostic_nslookup**](TelephonyProvidersEdgeApi.md#get_telephony_providers_edge_diagnostic_nslookup) | **GET** /api/v2/telephony/providers/edges/{edgeId}/diagnostic/nslookup | Get networking-related information from an Edge for a target IP or host.
[**get_telephony_providers_edge_diagnostic_ping**](TelephonyProvidersEdgeApi.md#get_telephony_providers_edge_diagnostic_ping) | **GET** /api/v2/telephony/providers/edges/{edgeId}/diagnostic/ping | Get networking-related information from an Edge for a target IP or host.
[**get_telephony_providers_edge_diagnostic_route**](TelephonyProvidersEdgeApi.md#get_telephony_providers_edge_diagnostic_route) | **GET** /api/v2/telephony/providers/edges/{edgeId}/diagnostic/route | Get networking-related information from an Edge for a target IP or host.
[**get_telephony_providers_edge_diagnostic_tracepath**](TelephonyProvidersEdgeApi.md#get_telephony_providers_edge_diagnostic_tracepath) | **GET** /api/v2/telephony/providers/edges/{edgeId}/diagnostic/tracepath | Get networking-related information from an Edge for a target IP or host.
[**get_telephony_providers_edge_line**](TelephonyProvidersEdgeApi.md#get_telephony_providers_edge_line) | **GET** /api/v2/telephony/providers/edges/{edgeId}/lines/{lineId} | Get line
[**get_telephony_providers_edge_lines**](TelephonyProvidersEdgeApi.md#get_telephony_providers_edge_lines) | **GET** /api/v2/telephony/providers/edges/{edgeId}/lines | Get the list of lines.
[**get_telephony_providers_edge_logicalinterface**](TelephonyProvidersEdgeApi.md#get_telephony_providers_edge_logicalinterface) | **GET** /api/v2/telephony/providers/edges/{edgeId}/logicalinterfaces/{interfaceId} | Get an edge logical interface
[**get_telephony_providers_edge_logicalinterfaces**](TelephonyProvidersEdgeApi.md#get_telephony_providers_edge_logicalinterfaces) | **GET** /api/v2/telephony/providers/edges/{edgeId}/logicalinterfaces | Get edge logical interfaces.
[**get_telephony_providers_edge_logs_job**](TelephonyProvidersEdgeApi.md#get_telephony_providers_edge_logs_job) | **GET** /api/v2/telephony/providers/edges/{edgeId}/logs/jobs/{jobId} | Get an Edge logs job.
[**get_telephony_providers_edge_metrics**](TelephonyProvidersEdgeApi.md#get_telephony_providers_edge_metrics) | **GET** /api/v2/telephony/providers/edges/{edgeId}/metrics | Get the edge metrics.
[**get_telephony_providers_edge_physicalinterface**](TelephonyProvidersEdgeApi.md#get_telephony_providers_edge_physicalinterface) | **GET** /api/v2/telephony/providers/edges/{edgeId}/physicalinterfaces/{interfaceId} | Get edge physical interface.
[**get_telephony_providers_edge_physicalinterfaces**](TelephonyProvidersEdgeApi.md#get_telephony_providers_edge_physicalinterfaces) | **GET** /api/v2/telephony/providers/edges/{edgeId}/physicalinterfaces | Retrieve a list of all configured physical interfaces from a specific edge.
[**get_telephony_providers_edge_setuppackage**](TelephonyProvidersEdgeApi.md#get_telephony_providers_edge_setuppackage) | **GET** /api/v2/telephony/providers/edges/{edgeId}/setuppackage | Get the setup package for a locally deployed edge device. This is needed to complete the setup process for the virtual edge.
[**get_telephony_providers_edge_softwareupdate**](TelephonyProvidersEdgeApi.md#get_telephony_providers_edge_softwareupdate) | **GET** /api/v2/telephony/providers/edges/{edgeId}/softwareupdate | Gets software update status information about any edge.
[**get_telephony_providers_edge_softwareversions**](TelephonyProvidersEdgeApi.md#get_telephony_providers_edge_softwareversions) | **GET** /api/v2/telephony/providers/edges/{edgeId}/softwareversions | Gets all the available software versions for this edge.
[**get_telephony_providers_edge_trunks**](TelephonyProvidersEdgeApi.md#get_telephony_providers_edge_trunks) | **GET** /api/v2/telephony/providers/edges/{edgeId}/trunks | Get the list of available trunks for the given Edge.
[**get_telephony_providers_edges**](TelephonyProvidersEdgeApi.md#get_telephony_providers_edges) | **GET** /api/v2/telephony/providers/edges | Get the list of edges.
[**get_telephony_providers_edges_availablelanguages**](TelephonyProvidersEdgeApi.md#get_telephony_providers_edges_availablelanguages) | **GET** /api/v2/telephony/providers/edges/availablelanguages | Get the list of available languages.
[**get_telephony_providers_edges_certificateauthorities**](TelephonyProvidersEdgeApi.md#get_telephony_providers_edges_certificateauthorities) | **GET** /api/v2/telephony/providers/edges/certificateauthorities | Get the list of certificate authorities.
[**get_telephony_providers_edges_certificateauthority**](TelephonyProvidersEdgeApi.md#get_telephony_providers_edges_certificateauthority) | **GET** /api/v2/telephony/providers/edges/certificateauthorities/{certificateId} | Get a certificate authority.
[**get_telephony_providers_edges_did**](TelephonyProvidersEdgeApi.md#get_telephony_providers_edges_did) | **GET** /api/v2/telephony/providers/edges/dids/{didId} | Get a DID by ID.
[**get_telephony_providers_edges_didpool**](TelephonyProvidersEdgeApi.md#get_telephony_providers_edges_didpool) | **GET** /api/v2/telephony/providers/edges/didpools/{didPoolId} | Get a DID Pool by ID.
[**get_telephony_providers_edges_didpools**](TelephonyProvidersEdgeApi.md#get_telephony_providers_edges_didpools) | **GET** /api/v2/telephony/providers/edges/didpools | Get a listing of DID Pools
[**get_telephony_providers_edges_didpools_dids**](TelephonyProvidersEdgeApi.md#get_telephony_providers_edges_didpools_dids) | **GET** /api/v2/telephony/providers/edges/didpools/dids | Get a listing of unassigned and/or assigned numbers in a set of DID Pools.
[**get_telephony_providers_edges_dids**](TelephonyProvidersEdgeApi.md#get_telephony_providers_edges_dids) | **GET** /api/v2/telephony/providers/edges/dids | Get a listing of DIDs
[**get_telephony_providers_edges_edgegroup**](TelephonyProvidersEdgeApi.md#get_telephony_providers_edges_edgegroup) | **GET** /api/v2/telephony/providers/edges/edgegroups/{edgeGroupId} | Get edge group.
[**get_telephony_providers_edges_edgegroup_edgetrunkbase**](TelephonyProvidersEdgeApi.md#get_telephony_providers_edges_edgegroup_edgetrunkbase) | **GET** /api/v2/telephony/providers/edges/edgegroups/{edgegroupId}/edgetrunkbases/{edgetrunkbaseId} | Gets the edge trunk base associated with the edge group
[**get_telephony_providers_edges_edgegroups**](TelephonyProvidersEdgeApi.md#get_telephony_providers_edges_edgegroups) | **GET** /api/v2/telephony/providers/edges/edgegroups | Get the list of edge groups.
[**get_telephony_providers_edges_edgeversionreport**](TelephonyProvidersEdgeApi.md#get_telephony_providers_edges_edgeversionreport) | **GET** /api/v2/telephony/providers/edges/edgeversionreport | Get the edge version report.
[**get_telephony_providers_edges_expired**](TelephonyProvidersEdgeApi.md#get_telephony_providers_edges_expired) | **GET** /api/v2/telephony/providers/edges/expired | List of edges more than 4 edge versions behind the latest software.
[**get_telephony_providers_edges_extension**](TelephonyProvidersEdgeApi.md#get_telephony_providers_edges_extension) | **GET** /api/v2/telephony/providers/edges/extensions/{extensionId} | Get an extension by ID.
[**get_telephony_providers_edges_extensionpool**](TelephonyProvidersEdgeApi.md#get_telephony_providers_edges_extensionpool) | **GET** /api/v2/telephony/providers/edges/extensionpools/{extensionPoolId} | Get an extension pool by ID
[**get_telephony_providers_edges_extensionpools**](TelephonyProvidersEdgeApi.md#get_telephony_providers_edges_extensionpools) | **GET** /api/v2/telephony/providers/edges/extensionpools | Get a listing of extension pools
[**get_telephony_providers_edges_extensions**](TelephonyProvidersEdgeApi.md#get_telephony_providers_edges_extensions) | **GET** /api/v2/telephony/providers/edges/extensions | Get a listing of extensions
[**get_telephony_providers_edges_line**](TelephonyProvidersEdgeApi.md#get_telephony_providers_edges_line) | **GET** /api/v2/telephony/providers/edges/lines/{lineId} | Get a Line by ID
[**get_telephony_providers_edges_linebasesetting**](TelephonyProvidersEdgeApi.md#get_telephony_providers_edges_linebasesetting) | **GET** /api/v2/telephony/providers/edges/linebasesettings/{lineBaseId} | Get a line base settings object by ID
[**get_telephony_providers_edges_linebasesettings**](TelephonyProvidersEdgeApi.md#get_telephony_providers_edges_linebasesettings) | **GET** /api/v2/telephony/providers/edges/linebasesettings | Get a listing of line base settings objects
[**get_telephony_providers_edges_lines**](TelephonyProvidersEdgeApi.md#get_telephony_providers_edges_lines) | **GET** /api/v2/telephony/providers/edges/lines | Get a list of Lines
[**get_telephony_providers_edges_lines_template**](TelephonyProvidersEdgeApi.md#get_telephony_providers_edges_lines_template) | **GET** /api/v2/telephony/providers/edges/lines/template | Get a Line instance template based on a Line Base Settings object. This object can then be modified and saved as a new Line instance
[**get_telephony_providers_edges_logicalinterfaces**](TelephonyProvidersEdgeApi.md#get_telephony_providers_edges_logicalinterfaces) | **GET** /api/v2/telephony/providers/edges/logicalinterfaces | Get edge logical interfaces.
[**get_telephony_providers_edges_metrics**](TelephonyProvidersEdgeApi.md#get_telephony_providers_edges_metrics) | **GET** /api/v2/telephony/providers/edges/metrics | Get the metrics for a list of edges.
[**get_telephony_providers_edges_outboundroute**](TelephonyProvidersEdgeApi.md#get_telephony_providers_edges_outboundroute) | **GET** /api/v2/telephony/providers/edges/outboundroutes/{outboundRouteId} | Get outbound route
[**get_telephony_providers_edges_outboundroutes**](TelephonyProvidersEdgeApi.md#get_telephony_providers_edges_outboundroutes) | **GET** /api/v2/telephony/providers/edges/outboundroutes | Get outbound routes
[**get_telephony_providers_edges_phone**](TelephonyProvidersEdgeApi.md#get_telephony_providers_edges_phone) | **GET** /api/v2/telephony/providers/edges/phones/{phoneId} | Get a Phone by ID
[**get_telephony_providers_edges_phonebasesetting**](TelephonyProvidersEdgeApi.md#get_telephony_providers_edges_phonebasesetting) | **GET** /api/v2/telephony/providers/edges/phonebasesettings/{phoneBaseId} | Get a Phone Base Settings object by ID
[**get_telephony_providers_edges_phonebasesettings**](TelephonyProvidersEdgeApi.md#get_telephony_providers_edges_phonebasesettings) | **GET** /api/v2/telephony/providers/edges/phonebasesettings | Get a list of Phone Base Settings objects
[**get_telephony_providers_edges_phonebasesettings_availablemetabases**](TelephonyProvidersEdgeApi.md#get_telephony_providers_edges_phonebasesettings_availablemetabases) | **GET** /api/v2/telephony/providers/edges/phonebasesettings/availablemetabases | Get a list of available makes and models to create a new Phone Base Settings
[**get_telephony_providers_edges_phonebasesettings_template**](TelephonyProvidersEdgeApi.md#get_telephony_providers_edges_phonebasesettings_template) | **GET** /api/v2/telephony/providers/edges/phonebasesettings/template | Get a Phone Base Settings instance template from a given make and model. This object can then be modified and saved as a new Phone Base Settings instance
[**get_telephony_providers_edges_phones**](TelephonyProvidersEdgeApi.md#get_telephony_providers_edges_phones) | **GET** /api/v2/telephony/providers/edges/phones | Get a list of Phone Instances
[**get_telephony_providers_edges_phones_template**](TelephonyProvidersEdgeApi.md#get_telephony_providers_edges_phones_template) | **GET** /api/v2/telephony/providers/edges/phones/template | Get a Phone instance template based on a Phone Base Settings object. This object can then be modified and saved as a new Phone instance
[**get_telephony_providers_edges_physicalinterfaces**](TelephonyProvidersEdgeApi.md#get_telephony_providers_edges_physicalinterfaces) | **GET** /api/v2/telephony/providers/edges/physicalinterfaces | Get physical interfaces for edges.
[**get_telephony_providers_edges_site**](TelephonyProvidersEdgeApi.md#get_telephony_providers_edges_site) | **GET** /api/v2/telephony/providers/edges/sites/{siteId} | Get a Site by ID.
[**get_telephony_providers_edges_site_numberplan**](TelephonyProvidersEdgeApi.md#get_telephony_providers_edges_site_numberplan) | **GET** /api/v2/telephony/providers/edges/sites/{siteId}/numberplans/{numberPlanId} | Get a Number Plan by ID.
[**get_telephony_providers_edges_site_numberplans**](TelephonyProvidersEdgeApi.md#get_telephony_providers_edges_site_numberplans) | **GET** /api/v2/telephony/providers/edges/sites/{siteId}/numberplans | Get the list of Number Plans for this Site. Only fetches the first 200 records.
[**get_telephony_providers_edges_site_numberplans_classifications**](TelephonyProvidersEdgeApi.md#get_telephony_providers_edges_site_numberplans_classifications) | **GET** /api/v2/telephony/providers/edges/sites/{siteId}/numberplans/classifications | Get a list of Classifications for this Site
[**get_telephony_providers_edges_site_outboundroute**](TelephonyProvidersEdgeApi.md#get_telephony_providers_edges_site_outboundroute) | **GET** /api/v2/telephony/providers/edges/sites/{siteId}/outboundroutes/{outboundRouteId} | Get an outbound route
[**get_telephony_providers_edges_site_outboundroutes**](TelephonyProvidersEdgeApi.md#get_telephony_providers_edges_site_outboundroutes) | **GET** /api/v2/telephony/providers/edges/sites/{siteId}/outboundroutes | Get outbound routes
[**get_telephony_providers_edges_sites**](TelephonyProvidersEdgeApi.md#get_telephony_providers_edges_sites) | **GET** /api/v2/telephony/providers/edges/sites | Get the list of Sites.
[**get_telephony_providers_edges_timezones**](TelephonyProvidersEdgeApi.md#get_telephony_providers_edges_timezones) | **GET** /api/v2/telephony/providers/edges/timezones | Get a list of Edge-compatible time zones
[**get_telephony_providers_edges_trunk**](TelephonyProvidersEdgeApi.md#get_telephony_providers_edges_trunk) | **GET** /api/v2/telephony/providers/edges/trunks/{trunkId} | Get a Trunk by ID
[**get_telephony_providers_edges_trunk_metrics**](TelephonyProvidersEdgeApi.md#get_telephony_providers_edges_trunk_metrics) | **GET** /api/v2/telephony/providers/edges/trunks/{trunkId}/metrics | Get the trunk metrics.
[**get_telephony_providers_edges_trunkbasesetting**](TelephonyProvidersEdgeApi.md#get_telephony_providers_edges_trunkbasesetting) | **GET** /api/v2/telephony/providers/edges/trunkbasesettings/{trunkBaseSettingsId} | Get a Trunk Base Settings object by ID
[**get_telephony_providers_edges_trunkbasesettings**](TelephonyProvidersEdgeApi.md#get_telephony_providers_edges_trunkbasesettings) | **GET** /api/v2/telephony/providers/edges/trunkbasesettings | Get Trunk Base Settings listing
[**get_telephony_providers_edges_trunkbasesettings_availablemetabases**](TelephonyProvidersEdgeApi.md#get_telephony_providers_edges_trunkbasesettings_availablemetabases) | **GET** /api/v2/telephony/providers/edges/trunkbasesettings/availablemetabases | Get a list of available makes and models to create a new Trunk Base Settings
[**get_telephony_providers_edges_trunkbasesettings_template**](TelephonyProvidersEdgeApi.md#get_telephony_providers_edges_trunkbasesettings_template) | **GET** /api/v2/telephony/providers/edges/trunkbasesettings/template | Get a Trunk Base Settings instance template from a given make and model. This object can then be modified and saved as a new Trunk Base Settings instance
[**get_telephony_providers_edges_trunks**](TelephonyProvidersEdgeApi.md#get_telephony_providers_edges_trunks) | **GET** /api/v2/telephony/providers/edges/trunks | Get the list of available trunks.
[**get_telephony_providers_edges_trunks_metrics**](TelephonyProvidersEdgeApi.md#get_telephony_providers_edges_trunks_metrics) | **GET** /api/v2/telephony/providers/edges/trunks/metrics | Get the metrics for a list of trunks.
[**get_telephony_providers_edges_trunkswithrecording**](TelephonyProvidersEdgeApi.md#get_telephony_providers_edges_trunkswithrecording) | **GET** /api/v2/telephony/providers/edges/trunkswithrecording | Get Counts of trunks that have recording disabled or enabled
[**post_telephony_providers_edge_diagnostic_nslookup**](TelephonyProvidersEdgeApi.md#post_telephony_providers_edge_diagnostic_nslookup) | **POST** /api/v2/telephony/providers/edges/{edgeId}/diagnostic/nslookup | Nslookup request command to collect networking-related information from an Edge for a target IP or host.
[**post_telephony_providers_edge_diagnostic_ping**](TelephonyProvidersEdgeApi.md#post_telephony_providers_edge_diagnostic_ping) | **POST** /api/v2/telephony/providers/edges/{edgeId}/diagnostic/ping | Ping Request command to collect networking-related information from an Edge for a target IP or host.
[**post_telephony_providers_edge_diagnostic_route**](TelephonyProvidersEdgeApi.md#post_telephony_providers_edge_diagnostic_route) | **POST** /api/v2/telephony/providers/edges/{edgeId}/diagnostic/route | Route request command to collect networking-related information from an Edge for a target IP or host.
[**post_telephony_providers_edge_diagnostic_tracepath**](TelephonyProvidersEdgeApi.md#post_telephony_providers_edge_diagnostic_tracepath) | **POST** /api/v2/telephony/providers/edges/{edgeId}/diagnostic/tracepath | Tracepath request command to collect networking-related information from an Edge for a target IP or host.
[**post_telephony_providers_edge_logicalinterfaces**](TelephonyProvidersEdgeApi.md#post_telephony_providers_edge_logicalinterfaces) | **POST** /api/v2/telephony/providers/edges/{edgeId}/logicalinterfaces | Create an edge logical interface.
[**post_telephony_providers_edge_logs_job_upload**](TelephonyProvidersEdgeApi.md#post_telephony_providers_edge_logs_job_upload) | **POST** /api/v2/telephony/providers/edges/{edgeId}/logs/jobs/{jobId}/upload | Request that the specified fileIds be uploaded from the Edge.
[**post_telephony_providers_edge_logs_jobs**](TelephonyProvidersEdgeApi.md#post_telephony_providers_edge_logs_jobs) | **POST** /api/v2/telephony/providers/edges/{edgeId}/logs/jobs | Create a job to upload a list of Edge logs.
[**post_telephony_providers_edge_reboot**](TelephonyProvidersEdgeApi.md#post_telephony_providers_edge_reboot) | **POST** /api/v2/telephony/providers/edges/{edgeId}/reboot | Reboot an Edge
[**post_telephony_providers_edge_softwareupdate**](TelephonyProvidersEdgeApi.md#post_telephony_providers_edge_softwareupdate) | **POST** /api/v2/telephony/providers/edges/{edgeId}/softwareupdate | Starts a software update for this edge.
[**post_telephony_providers_edge_statuscode**](TelephonyProvidersEdgeApi.md#post_telephony_providers_edge_statuscode) | **POST** /api/v2/telephony/providers/edges/{edgeId}/statuscode | Take an Edge in or out of service
[**post_telephony_providers_edge_unpair**](TelephonyProvidersEdgeApi.md#post_telephony_providers_edge_unpair) | **POST** /api/v2/telephony/providers/edges/{edgeId}/unpair | Unpair an Edge
[**post_telephony_providers_edges**](TelephonyProvidersEdgeApi.md#post_telephony_providers_edges) | **POST** /api/v2/telephony/providers/edges | Create an edge.
[**post_telephony_providers_edges_addressvalidation**](TelephonyProvidersEdgeApi.md#post_telephony_providers_edges_addressvalidation) | **POST** /api/v2/telephony/providers/edges/addressvalidation | Validates a street address
[**post_telephony_providers_edges_certificateauthorities**](TelephonyProvidersEdgeApi.md#post_telephony_providers_edges_certificateauthorities) | **POST** /api/v2/telephony/providers/edges/certificateauthorities | Create a certificate authority.
[**post_telephony_providers_edges_didpools**](TelephonyProvidersEdgeApi.md#post_telephony_providers_edges_didpools) | **POST** /api/v2/telephony/providers/edges/didpools | Create a new DID pool
[**post_telephony_providers_edges_edgegroups**](TelephonyProvidersEdgeApi.md#post_telephony_providers_edges_edgegroups) | **POST** /api/v2/telephony/providers/edges/edgegroups | Create an edge group.
[**post_telephony_providers_edges_extensionpools**](TelephonyProvidersEdgeApi.md#post_telephony_providers_edges_extensionpools) | **POST** /api/v2/telephony/providers/edges/extensionpools | Create a new extension pool
[**post_telephony_providers_edges_outboundroutes**](TelephonyProvidersEdgeApi.md#post_telephony_providers_edges_outboundroutes) | **POST** /api/v2/telephony/providers/edges/outboundroutes | Create outbound rule
[**post_telephony_providers_edges_phone_reboot**](TelephonyProvidersEdgeApi.md#post_telephony_providers_edges_phone_reboot) | **POST** /api/v2/telephony/providers/edges/phones/{phoneId}/reboot | Reboot a Phone
[**post_telephony_providers_edges_phonebasesettings**](TelephonyProvidersEdgeApi.md#post_telephony_providers_edges_phonebasesettings) | **POST** /api/v2/telephony/providers/edges/phonebasesettings | Create a new Phone Base Settings object
[**post_telephony_providers_edges_phones**](TelephonyProvidersEdgeApi.md#post_telephony_providers_edges_phones) | **POST** /api/v2/telephony/providers/edges/phones | Create a new Phone
[**post_telephony_providers_edges_phones_reboot**](TelephonyProvidersEdgeApi.md#post_telephony_providers_edges_phones_reboot) | **POST** /api/v2/telephony/providers/edges/phones/reboot | Reboot Multiple Phones
[**post_telephony_providers_edges_site_outboundroutes**](TelephonyProvidersEdgeApi.md#post_telephony_providers_edges_site_outboundroutes) | **POST** /api/v2/telephony/providers/edges/sites/{siteId}/outboundroutes | Create outbound route
[**post_telephony_providers_edges_site_rebalance**](TelephonyProvidersEdgeApi.md#post_telephony_providers_edges_site_rebalance) | **POST** /api/v2/telephony/providers/edges/sites/{siteId}/rebalance | Triggers the rebalance operation.
[**post_telephony_providers_edges_sites**](TelephonyProvidersEdgeApi.md#post_telephony_providers_edges_sites) | **POST** /api/v2/telephony/providers/edges/sites | Create a Site.
[**post_telephony_providers_edges_trunkbasesettings**](TelephonyProvidersEdgeApi.md#post_telephony_providers_edges_trunkbasesettings) | **POST** /api/v2/telephony/providers/edges/trunkbasesettings | Create a Trunk Base Settings object
[**put_telephony_providers_edge**](TelephonyProvidersEdgeApi.md#put_telephony_providers_edge) | **PUT** /api/v2/telephony/providers/edges/{edgeId} | Update a edge.
[**put_telephony_providers_edge_line**](TelephonyProvidersEdgeApi.md#put_telephony_providers_edge_line) | **PUT** /api/v2/telephony/providers/edges/{edgeId}/lines/{lineId} | Update a line.
[**put_telephony_providers_edge_logicalinterface**](TelephonyProvidersEdgeApi.md#put_telephony_providers_edge_logicalinterface) | **PUT** /api/v2/telephony/providers/edges/{edgeId}/logicalinterfaces/{interfaceId} | Update an edge logical interface.
[**put_telephony_providers_edges_certificateauthority**](TelephonyProvidersEdgeApi.md#put_telephony_providers_edges_certificateauthority) | **PUT** /api/v2/telephony/providers/edges/certificateauthorities/{certificateId} | Update a certificate authority.
[**put_telephony_providers_edges_did**](TelephonyProvidersEdgeApi.md#put_telephony_providers_edges_did) | **PUT** /api/v2/telephony/providers/edges/dids/{didId} | Update a DID by ID.
[**put_telephony_providers_edges_didpool**](TelephonyProvidersEdgeApi.md#put_telephony_providers_edges_didpool) | **PUT** /api/v2/telephony/providers/edges/didpools/{didPoolId} | Update a DID Pool by ID.
[**put_telephony_providers_edges_edgegroup**](TelephonyProvidersEdgeApi.md#put_telephony_providers_edges_edgegroup) | **PUT** /api/v2/telephony/providers/edges/edgegroups/{edgeGroupId} | Update an edge group.
[**put_telephony_providers_edges_edgegroup_edgetrunkbase**](TelephonyProvidersEdgeApi.md#put_telephony_providers_edges_edgegroup_edgetrunkbase) | **PUT** /api/v2/telephony/providers/edges/edgegroups/{edgegroupId}/edgetrunkbases/{edgetrunkbaseId} | Update the edge trunk base associated with the edge group
[**put_telephony_providers_edges_extension**](TelephonyProvidersEdgeApi.md#put_telephony_providers_edges_extension) | **PUT** /api/v2/telephony/providers/edges/extensions/{extensionId} | Update an extension by ID.
[**put_telephony_providers_edges_extensionpool**](TelephonyProvidersEdgeApi.md#put_telephony_providers_edges_extensionpool) | **PUT** /api/v2/telephony/providers/edges/extensionpools/{extensionPoolId} | Update an extension pool by ID
[**put_telephony_providers_edges_outboundroute**](TelephonyProvidersEdgeApi.md#put_telephony_providers_edges_outboundroute) | **PUT** /api/v2/telephony/providers/edges/outboundroutes/{outboundRouteId} | Update outbound route
[**put_telephony_providers_edges_phone**](TelephonyProvidersEdgeApi.md#put_telephony_providers_edges_phone) | **PUT** /api/v2/telephony/providers/edges/phones/{phoneId} | Update a Phone by ID
[**put_telephony_providers_edges_phonebasesetting**](TelephonyProvidersEdgeApi.md#put_telephony_providers_edges_phonebasesetting) | **PUT** /api/v2/telephony/providers/edges/phonebasesettings/{phoneBaseId} | Update a Phone Base Settings by ID
[**put_telephony_providers_edges_site**](TelephonyProvidersEdgeApi.md#put_telephony_providers_edges_site) | **PUT** /api/v2/telephony/providers/edges/sites/{siteId} | Update a Site by ID.
[**put_telephony_providers_edges_site_numberplans**](TelephonyProvidersEdgeApi.md#put_telephony_providers_edges_site_numberplans) | **PUT** /api/v2/telephony/providers/edges/sites/{siteId}/numberplans | Update the list of Number Plans. A user can update maximum 200 number plans at a time.
[**put_telephony_providers_edges_site_outboundroute**](TelephonyProvidersEdgeApi.md#put_telephony_providers_edges_site_outboundroute) | **PUT** /api/v2/telephony/providers/edges/sites/{siteId}/outboundroutes/{outboundRouteId} | Update outbound route
[**put_telephony_providers_edges_trunkbasesetting**](TelephonyProvidersEdgeApi.md#put_telephony_providers_edges_trunkbasesetting) | **PUT** /api/v2/telephony/providers/edges/trunkbasesettings/{trunkBaseSettingsId} | Update a Trunk Base Settings object by ID



## delete_telephony_providers_edge

> delete_telephony_providers_edge(edge_id)
Delete a edge.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edge_id** | **String** | Edge ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_telephony_providers_edge_logicalinterface

> delete_telephony_providers_edge_logicalinterface(edge_id, interface_id)
Delete an edge logical interface

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edge_id** | **String** | Edge ID | [required] |
**interface_id** | **String** | Interface ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_telephony_providers_edge_softwareupdate

> delete_telephony_providers_edge_softwareupdate(edge_id)
Cancels any in-progress update for this edge.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edge_id** | **String** | Edge ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_telephony_providers_edges_certificateauthority

> delete_telephony_providers_edges_certificateauthority(certificate_id)
Delete a certificate authority.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**certificate_id** | **String** | Certificate ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_telephony_providers_edges_didpool

> delete_telephony_providers_edges_didpool(did_pool_id)
Delete a DID Pool by ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did_pool_id** | **String** | DID pool ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_telephony_providers_edges_edgegroup

> delete_telephony_providers_edges_edgegroup(edge_group_id)
Delete an edge group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edge_group_id** | **String** | Edge group ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_telephony_providers_edges_extensionpool

> delete_telephony_providers_edges_extensionpool(extension_pool_id)
Delete an extension pool by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**extension_pool_id** | **String** | Extension pool ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_telephony_providers_edges_outboundroute

> delete_telephony_providers_edges_outboundroute(outbound_route_id)
Delete Outbound Route

This route is deprecated, use /telephony/providers/edges/sites/{siteId}/outboundroutes/{outboundRouteId} instead.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**outbound_route_id** | **String** | Outbound route ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_telephony_providers_edges_phone

> delete_telephony_providers_edges_phone(phone_id)
Delete a Phone by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**phone_id** | **String** | Phone ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_telephony_providers_edges_phonebasesetting

> delete_telephony_providers_edges_phonebasesetting(phone_base_id)
Delete a Phone Base Settings by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**phone_base_id** | **String** | Phone base ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_telephony_providers_edges_site

> delete_telephony_providers_edges_site(site_id)
Delete a Site by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** | Site ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_telephony_providers_edges_site_outboundroute

> delete_telephony_providers_edges_site_outboundroute(site_id, outbound_route_id)
Delete Outbound Route

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** | Site ID | [required] |
**outbound_route_id** | **String** | Outbound route ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_telephony_providers_edges_trunkbasesetting

> delete_telephony_providers_edges_trunkbasesetting(trunk_base_settings_id)
Delete a Trunk Base Settings object by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**trunk_base_settings_id** | **String** | Trunk Base ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_configuration_schemas_edges_vnext

> crate::models::SchemaCategoryEntityListing get_configuration_schemas_edges_vnext(page_size, page_number)
Lists available schema categories (Deprecated)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]

### Return type

[**crate::models::SchemaCategoryEntityListing**](SchemaCategoryEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_configuration_schemas_edges_vnext_schema_category

> crate::models::SchemaReferenceEntityListing get_configuration_schemas_edges_vnext_schema_category(schema_category, page_size, page_number)
List schemas of a specific category (Deprecated)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**schema_category** | **String** | Schema category | [required] |
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]

### Return type

[**crate::models::SchemaReferenceEntityListing**](SchemaReferenceEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_configuration_schemas_edges_vnext_schema_category_schema_type

> crate::models::SchemaReferenceEntityListing get_configuration_schemas_edges_vnext_schema_category_schema_type(schema_category, schema_type, page_size, page_number)
List schemas of a specific category (Deprecated)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**schema_category** | **String** | Schema category | [required] |
**schema_type** | **String** | Schema type | [required] |
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]

### Return type

[**crate::models::SchemaReferenceEntityListing**](SchemaReferenceEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_configuration_schemas_edges_vnext_schema_category_schema_type_schema_id

> crate::models::Organization get_configuration_schemas_edges_vnext_schema_category_schema_type_schema_id(schema_category, schema_type, schema_id)
Get a json schema (Deprecated)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**schema_category** | **String** | Schema category | [required] |
**schema_type** | **String** | Schema type | [required] |
**schema_id** | **String** | Schema ID | [required] |

### Return type

[**crate::models::Organization**](Organization.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_configuration_schemas_edges_vnext_schema_category_schema_type_schema_id_extension_type_metadata_id

> crate::models::Organization get_configuration_schemas_edges_vnext_schema_category_schema_type_schema_id_extension_type_metadata_id(schema_category, schema_type, schema_id, extension_type, metadata_id, _type)
Get metadata for a schema (Deprecated)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**schema_category** | **String** | Schema category | [required] |
**schema_type** | **String** | Schema type | [required] |
**schema_id** | **String** | Schema ID | [required] |
**extension_type** | **String** | extension | [required] |
**metadata_id** | **String** | Metadata ID | [required] |
**_type** | Option<**String**> | Type |  |

### Return type

[**crate::models::Organization**](Organization.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_telephony_providers_edge

> crate::models::Edge get_telephony_providers_edge(edge_id, expand)
Get edge.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edge_id** | **String** | Edge ID | [required] |
**expand** | Option<[**Vec<String>**](String.md)> | Fields to expand in the response, comma-separated |  |

### Return type

[**crate::models::Edge**](Edge.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_telephony_providers_edge_diagnostic_nslookup

> crate::models::EdgeNetworkDiagnosticResponse get_telephony_providers_edge_diagnostic_nslookup(edge_id)
Get networking-related information from an Edge for a target IP or host.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edge_id** | **String** | Edge Id | [required] |

### Return type

[**crate::models::EdgeNetworkDiagnosticResponse**](EdgeNetworkDiagnosticResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_telephony_providers_edge_diagnostic_ping

> crate::models::EdgeNetworkDiagnosticResponse get_telephony_providers_edge_diagnostic_ping(edge_id)
Get networking-related information from an Edge for a target IP or host.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edge_id** | **String** | Edge Id | [required] |

### Return type

[**crate::models::EdgeNetworkDiagnosticResponse**](EdgeNetworkDiagnosticResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_telephony_providers_edge_diagnostic_route

> crate::models::EdgeNetworkDiagnosticResponse get_telephony_providers_edge_diagnostic_route(edge_id)
Get networking-related information from an Edge for a target IP or host.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edge_id** | **String** | Edge Id | [required] |

### Return type

[**crate::models::EdgeNetworkDiagnosticResponse**](EdgeNetworkDiagnosticResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_telephony_providers_edge_diagnostic_tracepath

> crate::models::EdgeNetworkDiagnosticResponse get_telephony_providers_edge_diagnostic_tracepath(edge_id)
Get networking-related information from an Edge for a target IP or host.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edge_id** | **String** | Edge Id | [required] |

### Return type

[**crate::models::EdgeNetworkDiagnosticResponse**](EdgeNetworkDiagnosticResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_telephony_providers_edge_line

> crate::models::EdgeLine get_telephony_providers_edge_line(edge_id, line_id)
Get line

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edge_id** | **String** | Edge ID | [required] |
**line_id** | **String** | Line ID | [required] |

### Return type

[**crate::models::EdgeLine**](EdgeLine.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_telephony_providers_edge_lines

> crate::models::EdgeLineEntityListing get_telephony_providers_edge_lines(edge_id, page_size, page_number)
Get the list of lines.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edge_id** | **String** | Edge ID | [required] |
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]

### Return type

[**crate::models::EdgeLineEntityListing**](EdgeLineEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_telephony_providers_edge_logicalinterface

> crate::models::DomainLogicalInterface get_telephony_providers_edge_logicalinterface(edge_id, interface_id, expand)
Get an edge logical interface

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edge_id** | **String** | Edge ID | [required] |
**interface_id** | **String** | Interface ID | [required] |
**expand** | Option<[**Vec<String>**](String.md)> | Field to expand in the response |  |

### Return type

[**crate::models::DomainLogicalInterface**](DomainLogicalInterface.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_telephony_providers_edge_logicalinterfaces

> crate::models::LogicalInterfaceEntityListing get_telephony_providers_edge_logicalinterfaces(edge_id, expand)
Get edge logical interfaces.

Retrieve a list of all configured logical interfaces from a specific edge.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edge_id** | **String** | Edge ID | [required] |
**expand** | Option<[**Vec<String>**](String.md)> | Field to expand in the response |  |

### Return type

[**crate::models::LogicalInterfaceEntityListing**](LogicalInterfaceEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_telephony_providers_edge_logs_job

> crate::models::EdgeLogsJob get_telephony_providers_edge_logs_job(edge_id, job_id)
Get an Edge logs job.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edge_id** | **String** | Edge ID | [required] |
**job_id** | **String** | Job ID | [required] |

### Return type

[**crate::models::EdgeLogsJob**](EdgeLogsJob.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_telephony_providers_edge_metrics

> crate::models::EdgeMetrics get_telephony_providers_edge_metrics(edge_id)
Get the edge metrics.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edge_id** | **String** | Edge Id | [required] |

### Return type

[**crate::models::EdgeMetrics**](EdgeMetrics.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_telephony_providers_edge_physicalinterface

> crate::models::DomainPhysicalInterface get_telephony_providers_edge_physicalinterface(edge_id, interface_id)
Get edge physical interface.

Retrieve a physical interface from a specific edge.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edge_id** | **String** | Edge ID | [required] |
**interface_id** | **String** | Interface ID | [required] |

### Return type

[**crate::models::DomainPhysicalInterface**](DomainPhysicalInterface.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_telephony_providers_edge_physicalinterfaces

> crate::models::PhysicalInterfaceEntityListing get_telephony_providers_edge_physicalinterfaces(edge_id)
Retrieve a list of all configured physical interfaces from a specific edge.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edge_id** | **String** | Edge ID | [required] |

### Return type

[**crate::models::PhysicalInterfaceEntityListing**](PhysicalInterfaceEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_telephony_providers_edge_setuppackage

> crate::models::VmPairingInfo get_telephony_providers_edge_setuppackage(edge_id)
Get the setup package for a locally deployed edge device. This is needed to complete the setup process for the virtual edge.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edge_id** | **String** | Edge ID | [required] |

### Return type

[**crate::models::VmPairingInfo**](VmPairingInfo.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_telephony_providers_edge_softwareupdate

> crate::models::DomainEdgeSoftwareUpdateDto get_telephony_providers_edge_softwareupdate(edge_id)
Gets software update status information about any edge.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edge_id** | **String** | Edge ID | [required] |

### Return type

[**crate::models::DomainEdgeSoftwareUpdateDto**](DomainEdgeSoftwareUpdateDto.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_telephony_providers_edge_softwareversions

> crate::models::DomainEdgeSoftwareVersionDtoEntityListing get_telephony_providers_edge_softwareversions(edge_id)
Gets all the available software versions for this edge.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edge_id** | **String** | Edge ID | [required] |

### Return type

[**crate::models::DomainEdgeSoftwareVersionDtoEntityListing**](DomainEdgeSoftwareVersionDtoEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_telephony_providers_edge_trunks

> crate::models::TrunkEntityListing get_telephony_providers_edge_trunks(edge_id, page_number, page_size, sort_by, sort_order, trunk_base_id, trunk_type)
Get the list of available trunks for the given Edge.

Trunks are created by assigning trunk base settings to an Edge or Edge Group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edge_id** | **String** | Edge ID | [required] |
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**sort_by** | Option<**String**> | Value by which to sort |  |[default to name]
**sort_order** | Option<**String**> | Sort order |  |[default to ASC]
**trunk_base_id** | Option<**String**> | Filter by Trunk Base Ids |  |
**trunk_type** | Option<**String**> | Filter by a Trunk type |  |

### Return type

[**crate::models::TrunkEntityListing**](TrunkEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_telephony_providers_edges

> crate::models::EdgeEntityListing get_telephony_providers_edges(page_size, page_number, name, site_id, edge_group_id, sort_by, managed)
Get the list of edges.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**name** | Option<**String**> | Name |  |
**site_id** | Option<**String**> | Filter by site.id |  |
**edge_group_id** | Option<**String**> | Filter by edgeGroup.id |  |
**sort_by** | Option<**String**> | Sort by |  |[default to name]
**managed** | Option<**bool**> | Filter by managed |  |

### Return type

[**crate::models::EdgeEntityListing**](EdgeEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_telephony_providers_edges_availablelanguages

> crate::models::AvailableLanguageList get_telephony_providers_edges_availablelanguages()
Get the list of available languages.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::AvailableLanguageList**](AvailableLanguageList.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_telephony_providers_edges_certificateauthorities

> crate::models::CertificateAuthorityEntityListing get_telephony_providers_edges_certificateauthorities()
Get the list of certificate authorities.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CertificateAuthorityEntityListing**](CertificateAuthorityEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_telephony_providers_edges_certificateauthority

> crate::models::DomainCertificateAuthority get_telephony_providers_edges_certificateauthority(certificate_id)
Get a certificate authority.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**certificate_id** | **String** | Certificate ID | [required] |

### Return type

[**crate::models::DomainCertificateAuthority**](DomainCertificateAuthority.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_telephony_providers_edges_did

> crate::models::Did get_telephony_providers_edges_did(did_id)
Get a DID by ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did_id** | **String** | DID ID | [required] |

### Return type

[**crate::models::Did**](DID.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_telephony_providers_edges_didpool

> crate::models::DidPool get_telephony_providers_edges_didpool(did_pool_id)
Get a DID Pool by ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did_pool_id** | **String** | DID pool ID | [required] |

### Return type

[**crate::models::DidPool**](DIDPool.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_telephony_providers_edges_didpools

> crate::models::DidPoolEntityListing get_telephony_providers_edges_didpools(page_size, page_number, sort_by, id)
Get a listing of DID Pools

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**sort_by** | Option<**String**> | Sort by |  |[default to number]
**id** | Option<[**Vec<String>**](String.md)> | Filter by a specific list of ID's |  |

### Return type

[**crate::models::DidPoolEntityListing**](DIDPoolEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_telephony_providers_edges_didpools_dids

> crate::models::DidNumberEntityListing get_telephony_providers_edges_didpools_dids(_type, id, number_match, page_size, page_number, sort_order)
Get a listing of unassigned and/or assigned numbers in a set of DID Pools.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**_type** | **String** | The type of numbers to return. | [required] |
**id** | Option<[**Vec<String>**](String.md)> | Filter by a specific list of DID Pools.  If this is not provided, numbers from all DID Pools will be returned. |  |
**number_match** | Option<**String**> | A number to filter the results by. |  |
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**sort_order** | Option<**String**> | Sort order |  |[default to ascending]

### Return type

[**crate::models::DidNumberEntityListing**](DIDNumberEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_telephony_providers_edges_dids

> crate::models::DidEntityListing get_telephony_providers_edges_dids(page_size, page_number, sort_by, sort_order, phone_number, owner_id, did_pool_id, id)
Get a listing of DIDs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**sort_by** | Option<**String**> | Sort by |  |[default to number]
**sort_order** | Option<**String**> | Sort order |  |[default to ASC]
**phone_number** | Option<**String**> | Filter by phoneNumber |  |
**owner_id** | Option<**String**> | Filter by the owner of a phone number |  |
**did_pool_id** | Option<**String**> | Filter by the DID Pool assignment |  |
**id** | Option<[**Vec<String>**](String.md)> | Filter by a specific list of ID's |  |

### Return type

[**crate::models::DidEntityListing**](DIDEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_telephony_providers_edges_edgegroup

> crate::models::EdgeGroup get_telephony_providers_edges_edgegroup(edge_group_id, expand)
Get edge group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edge_group_id** | **String** | Edge group ID | [required] |
**expand** | Option<[**Vec<String>**](String.md)> | Fields to expand in the response |  |

### Return type

[**crate::models::EdgeGroup**](EdgeGroup.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_telephony_providers_edges_edgegroup_edgetrunkbase

> crate::models::EdgeTrunkBase get_telephony_providers_edges_edgegroup_edgetrunkbase(edgegroup_id, edgetrunkbase_id)
Gets the edge trunk base associated with the edge group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edgegroup_id** | **String** | Edge Group ID | [required] |
**edgetrunkbase_id** | **String** | Edge Trunk Base ID | [required] |

### Return type

[**crate::models::EdgeTrunkBase**](EdgeTrunkBase.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_telephony_providers_edges_edgegroups

> crate::models::EdgeGroupEntityListing get_telephony_providers_edges_edgegroups(page_size, page_number, name, sort_by, managed)
Get the list of edge groups.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**name** | Option<**String**> | Name |  |
**sort_by** | Option<**String**> | Sort by |  |[default to name]
**managed** | Option<**bool**> | Filter by managed |  |

### Return type

[**crate::models::EdgeGroupEntityListing**](EdgeGroupEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_telephony_providers_edges_edgeversionreport

> crate::models::EdgeVersionReport get_telephony_providers_edges_edgeversionreport()
Get the edge version report.

The report will not have consistent data about the edge version(s) until all edges have been reset.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::EdgeVersionReport**](EdgeVersionReport.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_telephony_providers_edges_expired

> crate::models::ExpiredEdgeListing get_telephony_providers_edges_expired()
List of edges more than 4 edge versions behind the latest software.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ExpiredEdgeListing**](ExpiredEdgeListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_telephony_providers_edges_extension

> crate::models::Extension get_telephony_providers_edges_extension(extension_id)
Get an extension by ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**extension_id** | **String** | Extension ID | [required] |

### Return type

[**crate::models::Extension**](Extension.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_telephony_providers_edges_extensionpool

> crate::models::ExtensionPool get_telephony_providers_edges_extensionpool(extension_pool_id)
Get an extension pool by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**extension_pool_id** | **String** | Extension pool ID | [required] |

### Return type

[**crate::models::ExtensionPool**](ExtensionPool.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_telephony_providers_edges_extensionpools

> crate::models::ExtensionPoolEntityListing get_telephony_providers_edges_extensionpools(page_size, page_number, sort_by, number)
Get a listing of extension pools

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**sort_by** | Option<**String**> | Sort by |  |
**number** | Option<**String**> | Number |  |

### Return type

[**crate::models::ExtensionPoolEntityListing**](ExtensionPoolEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_telephony_providers_edges_extensions

> crate::models::ExtensionEntityListing get_telephony_providers_edges_extensions(page_size, page_number, sort_by, sort_order, number)
Get a listing of extensions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**sort_by** | Option<**String**> | Sort by |  |[default to number]
**sort_order** | Option<**String**> | Sort order |  |[default to ASC]
**number** | Option<**String**> | Filter by number |  |

### Return type

[**crate::models::ExtensionEntityListing**](ExtensionEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_telephony_providers_edges_line

> crate::models::Line get_telephony_providers_edges_line(line_id)
Get a Line by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**line_id** | **String** | Line ID | [required] |

### Return type

[**crate::models::Line**](Line.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_telephony_providers_edges_linebasesetting

> crate::models::LineBase get_telephony_providers_edges_linebasesetting(line_base_id)
Get a line base settings object by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**line_base_id** | **String** | Line base ID | [required] |

### Return type

[**crate::models::LineBase**](LineBase.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_telephony_providers_edges_linebasesettings

> crate::models::LineBaseEntityListing get_telephony_providers_edges_linebasesettings(page_number, page_size, sort_by, sort_order, expand)
Get a listing of line base settings objects

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**sort_by** | Option<**String**> | Value by which to sort |  |[default to name]
**sort_order** | Option<**String**> | Sort order |  |[default to ASC]
**expand** | Option<[**Vec<String>**](String.md)> | Fields to expand in the response, comma-separated |  |

### Return type

[**crate::models::LineBaseEntityListing**](LineBaseEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_telephony_providers_edges_lines

> crate::models::LineEntityListing get_telephony_providers_edges_lines(page_size, page_number, name, sort_by, expand)
Get a list of Lines

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**name** | Option<**String**> | Name |  |
**sort_by** | Option<**String**> | Value by which to sort |  |[default to name]
**expand** | Option<[**Vec<String>**](String.md)> | Fields to expand in the response, comma-separated |  |

### Return type

[**crate::models::LineEntityListing**](LineEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_telephony_providers_edges_lines_template

> crate::models::Line get_telephony_providers_edges_lines_template(line_base_settings_id)
Get a Line instance template based on a Line Base Settings object. This object can then be modified and saved as a new Line instance

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**line_base_settings_id** | **String** | The id of a Line Base Settings object upon which to base this Line | [required] |

### Return type

[**crate::models::Line**](Line.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_telephony_providers_edges_logicalinterfaces

> crate::models::LogicalInterfaceEntityListing get_telephony_providers_edges_logicalinterfaces(edge_ids, expand)
Get edge logical interfaces.

Retrieve the configured logical interfaces for a list edges. Only 100 edges can be requested at a time.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edge_ids** | **String** | Comma separated list of Edge Id's | [required] |
**expand** | Option<[**Vec<String>**](String.md)> | Field to expand in the response |  |

### Return type

[**crate::models::LogicalInterfaceEntityListing**](LogicalInterfaceEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_telephony_providers_edges_metrics

> Vec<crate::models::EdgeMetrics> get_telephony_providers_edges_metrics(edge_ids)
Get the metrics for a list of edges.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edge_ids** | **String** | Comma separated list of Edge Id's | [required] |

### Return type

[**Vec<crate::models::EdgeMetrics>**](EdgeMetrics.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_telephony_providers_edges_outboundroute

> crate::models::OutboundRoute get_telephony_providers_edges_outboundroute(outbound_route_id)
Get outbound route

This route is deprecated, use /telephony/providers/edges/sites/{siteId}/outboundroutes/{outboundRouteId} instead.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**outbound_route_id** | **String** | Outbound route ID | [required] |

### Return type

[**crate::models::OutboundRoute**](OutboundRoute.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_telephony_providers_edges_outboundroutes

> crate::models::OutboundRouteEntityListing get_telephony_providers_edges_outboundroutes(page_size, page_number, name, site_id, external_trunk_bases_ids, sort_by)
Get outbound routes

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**name** | Option<**String**> | Name |  |
**site_id** | Option<**String**> | Filter by site.id |  |
**external_trunk_bases_ids** | Option<**String**> | Filter by externalTrunkBases.ids |  |
**sort_by** | Option<**String**> | Sort by |  |[default to name]

### Return type

[**crate::models::OutboundRouteEntityListing**](OutboundRouteEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_telephony_providers_edges_phone

> crate::models::Phone get_telephony_providers_edges_phone(phone_id)
Get a Phone by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**phone_id** | **String** | Phone ID | [required] |

### Return type

[**crate::models::Phone**](Phone.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_telephony_providers_edges_phonebasesetting

> crate::models::PhoneBase get_telephony_providers_edges_phonebasesetting(phone_base_id)
Get a Phone Base Settings object by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**phone_base_id** | **String** | Phone base ID | [required] |

### Return type

[**crate::models::PhoneBase**](PhoneBase.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_telephony_providers_edges_phonebasesettings

> crate::models::PhoneBaseEntityListing get_telephony_providers_edges_phonebasesettings(page_size, page_number, sort_by, sort_order, expand, name)
Get a list of Phone Base Settings objects

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**sort_by** | Option<**String**> | Value by which to sort |  |[default to name]
**sort_order** | Option<**String**> | Sort order |  |[default to ASC]
**expand** | Option<[**Vec<String>**](String.md)> | Fields to expand in the response, comma-separated |  |
**name** | Option<**String**> | Name |  |

### Return type

[**crate::models::PhoneBaseEntityListing**](PhoneBaseEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_telephony_providers_edges_phonebasesettings_availablemetabases

> crate::models::PhoneMetaBaseEntityListing get_telephony_providers_edges_phonebasesettings_availablemetabases(page_size, page_number)
Get a list of available makes and models to create a new Phone Base Settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]

### Return type

[**crate::models::PhoneMetaBaseEntityListing**](PhoneMetaBaseEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_telephony_providers_edges_phonebasesettings_template

> crate::models::PhoneBase get_telephony_providers_edges_phonebasesettings_template(phone_metabase_id)
Get a Phone Base Settings instance template from a given make and model. This object can then be modified and saved as a new Phone Base Settings instance

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**phone_metabase_id** | **String** | The id of a metabase object upon which to base this Phone Base Settings | [required] |

### Return type

[**crate::models::PhoneBase**](PhoneBase.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_telephony_providers_edges_phones

> crate::models::PhoneEntityListing get_telephony_providers_edges_phones(page_number, page_size, sort_by, sort_order, site_id, web_rtc_user_id, phone_base_settings_id, lines_logged_in_user_id, lines_default_for_user_id, phone_hardware_id, lines_id, lines_name, name, status_operational_status, secondary_status_operational_status, expand, fields)
Get a list of Phone Instances

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**sort_by** | Option<**String**> | The field to sort by |  |[default to name]
**sort_order** | Option<**String**> | Sort order |  |[default to ASC]
**site_id** | Option<**String**> | Filter by site.id |  |
**web_rtc_user_id** | Option<**String**> | Filter by webRtcUser.id |  |
**phone_base_settings_id** | Option<**String**> | Filter by phoneBaseSettings.id |  |
**lines_logged_in_user_id** | Option<**String**> | Filter by lines.loggedInUser.id |  |
**lines_default_for_user_id** | Option<**String**> | Filter by lines.defaultForUser.id |  |
**phone_hardware_id** | Option<**String**> | Filter by phone_hardwareId |  |
**lines_id** | Option<**String**> | Filter by lines.id |  |
**lines_name** | Option<**String**> | Filter by lines.name |  |
**name** | Option<**String**> | Name of the Phone to filter by |  |
**status_operational_status** | Option<**String**> | The primary status to filter by |  |
**secondary_status_operational_status** | Option<**String**> | The secondary status to filter by |  |
**expand** | Option<[**Vec<String>**](String.md)> | Fields to expand in the response, comma-separated |  |
**fields** | Option<[**Vec<String>**](String.md)> | Fields and properties to get, comma-separated |  |

### Return type

[**crate::models::PhoneEntityListing**](PhoneEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_telephony_providers_edges_phones_template

> crate::models::Phone get_telephony_providers_edges_phones_template(phone_base_settings_id)
Get a Phone instance template based on a Phone Base Settings object. This object can then be modified and saved as a new Phone instance

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**phone_base_settings_id** | **String** | The id of a Phone Base Settings object upon which to base this Phone | [required] |

### Return type

[**crate::models::Phone**](Phone.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_telephony_providers_edges_physicalinterfaces

> crate::models::PhysicalInterfaceEntityListing get_telephony_providers_edges_physicalinterfaces(edge_ids)
Get physical interfaces for edges.

Retrieves a list of all configured physical interfaces for a list of edges. Only 100 edges can be requested at a time.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edge_ids** | **String** | Comma separated list of Edge Id's | [required] |

### Return type

[**crate::models::PhysicalInterfaceEntityListing**](PhysicalInterfaceEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_telephony_providers_edges_site

> crate::models::Site get_telephony_providers_edges_site(site_id)
Get a Site by ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** | Site ID | [required] |

### Return type

[**crate::models::Site**](Site.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_telephony_providers_edges_site_numberplan

> crate::models::NumberPlan get_telephony_providers_edges_site_numberplan(site_id, number_plan_id)
Get a Number Plan by ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** | Site ID | [required] |
**number_plan_id** | **String** | Number Plan ID | [required] |

### Return type

[**crate::models::NumberPlan**](NumberPlan.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_telephony_providers_edges_site_numberplans

> Vec<crate::models::NumberPlan> get_telephony_providers_edges_site_numberplans(site_id)
Get the list of Number Plans for this Site. Only fetches the first 200 records.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** | Site ID | [required] |

### Return type

[**Vec<crate::models::NumberPlan>**](NumberPlan.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_telephony_providers_edges_site_numberplans_classifications

> Vec<String> get_telephony_providers_edges_site_numberplans_classifications(site_id, classification)
Get a list of Classifications for this Site

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** | Site ID | [required] |
**classification** | Option<**String**> | Classification |  |

### Return type

**Vec<String>**

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_telephony_providers_edges_site_outboundroute

> crate::models::OutboundRouteBase get_telephony_providers_edges_site_outboundroute(site_id, outbound_route_id)
Get an outbound route

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** | Site ID | [required] |
**outbound_route_id** | **String** | Outbound route ID | [required] |

### Return type

[**crate::models::OutboundRouteBase**](OutboundRouteBase.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_telephony_providers_edges_site_outboundroutes

> crate::models::OutboundRouteBaseEntityListing get_telephony_providers_edges_site_outboundroutes(site_id, page_size, page_number, name, external_trunk_bases_ids, sort_by)
Get outbound routes

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** | Site ID | [required] |
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**name** | Option<**String**> | Name |  |
**external_trunk_bases_ids** | Option<**String**> | externalTrunkBases.ids |  |
**sort_by** | Option<**String**> | Sort by |  |[default to name]

### Return type

[**crate::models::OutboundRouteBaseEntityListing**](OutboundRouteBaseEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_telephony_providers_edges_sites

> crate::models::SiteEntityListing get_telephony_providers_edges_sites(page_size, page_number, sort_by, sort_order, name, location_id, managed)
Get the list of Sites.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**sort_by** | Option<**String**> | Sort by |  |[default to name]
**sort_order** | Option<**String**> | Sort order |  |[default to ASC]
**name** | Option<**String**> | Name |  |
**location_id** | Option<**String**> | Location Id |  |
**managed** | Option<**bool**> | Filter by managed |  |

### Return type

[**crate::models::SiteEntityListing**](SiteEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_telephony_providers_edges_timezones

> crate::models::TimeZoneEntityListing get_telephony_providers_edges_timezones(page_size, page_number)
Get a list of Edge-compatible time zones

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | Page size |  |[default to 1000]
**page_number** | Option<**i32**> | Page number |  |[default to 1]

### Return type

[**crate::models::TimeZoneEntityListing**](TimeZoneEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_telephony_providers_edges_trunk

> crate::models::Trunk get_telephony_providers_edges_trunk(trunk_id)
Get a Trunk by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**trunk_id** | **String** | Trunk ID | [required] |

### Return type

[**crate::models::Trunk**](Trunk.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_telephony_providers_edges_trunk_metrics

> crate::models::TrunkMetrics get_telephony_providers_edges_trunk_metrics(trunk_id)
Get the trunk metrics.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**trunk_id** | **String** | Trunk Id | [required] |

### Return type

[**crate::models::TrunkMetrics**](TrunkMetrics.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_telephony_providers_edges_trunkbasesetting

> crate::models::TrunkBase get_telephony_providers_edges_trunkbasesetting(trunk_base_settings_id, ignore_hidden)
Get a Trunk Base Settings object by ID

Managed properties will not be returned unless the user is assigned the internal:trunk:edit permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**trunk_base_settings_id** | **String** | Trunk Base ID | [required] |
**ignore_hidden** | Option<**bool**> | Set this to true to not receive trunk properties that are meant to be hidden or for internal system usage only. |  |

### Return type

[**crate::models::TrunkBase**](TrunkBase.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_telephony_providers_edges_trunkbasesettings

> crate::models::TrunkBaseEntityListing get_telephony_providers_edges_trunkbasesettings(page_number, page_size, sort_by, sort_order, recording_enabled, ignore_hidden, managed, expand, name)
Get Trunk Base Settings listing

Managed properties will not be returned unless the user is assigned the internal:trunk:edit permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**sort_by** | Option<**String**> | Value by which to sort |  |[default to name]
**sort_order** | Option<**String**> | Sort order |  |[default to ASC]
**recording_enabled** | Option<**bool**> | Filter trunks by recording enabled |  |
**ignore_hidden** | Option<**bool**> | Set this to true to not receive trunk properties that are meant to be hidden or for internal system usage only. |  |
**managed** | Option<**bool**> | Filter by managed |  |
**expand** | Option<[**Vec<String>**](String.md)> | Fields to expand in the response, comma-separated |  |
**name** | Option<**String**> | Name of the TrunkBase to filter by |  |

### Return type

[**crate::models::TrunkBaseEntityListing**](TrunkBaseEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_telephony_providers_edges_trunkbasesettings_availablemetabases

> crate::models::TrunkMetabaseEntityListing get_telephony_providers_edges_trunkbasesettings_availablemetabases(_type, page_size, page_number)
Get a list of available makes and models to create a new Trunk Base Settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**_type** | Option<**String**> |  |  |
**page_size** | Option<**i32**> |  |  |[default to 25]
**page_number** | Option<**i32**> |  |  |[default to 1]

### Return type

[**crate::models::TrunkMetabaseEntityListing**](TrunkMetabaseEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_telephony_providers_edges_trunkbasesettings_template

> crate::models::TrunkBase get_telephony_providers_edges_trunkbasesettings_template(trunk_metabase_id)
Get a Trunk Base Settings instance template from a given make and model. This object can then be modified and saved as a new Trunk Base Settings instance

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**trunk_metabase_id** | **String** | The id of a metabase object upon which to base this Trunk Base Settings | [required] |

### Return type

[**crate::models::TrunkBase**](TrunkBase.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_telephony_providers_edges_trunks

> crate::models::TrunkEntityListing get_telephony_providers_edges_trunks(page_number, page_size, sort_by, sort_order, edge_id, trunk_base_id, trunk_type)
Get the list of available trunks.

Trunks are created by assigning trunk base settings to an Edge or Edge Group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**sort_by** | Option<**String**> | Value by which to sort |  |[default to name]
**sort_order** | Option<**String**> | Sort order |  |[default to ASC]
**edge_id** | Option<**String**> | Filter by Edge Ids |  |
**trunk_base_id** | Option<**String**> | Filter by Trunk Base Ids |  |
**trunk_type** | Option<**String**> | Filter by a Trunk type |  |

### Return type

[**crate::models::TrunkEntityListing**](TrunkEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_telephony_providers_edges_trunks_metrics

> Vec<crate::models::TrunkMetrics> get_telephony_providers_edges_trunks_metrics(trunk_ids)
Get the metrics for a list of trunks.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**trunk_ids** | **String** | Comma separated list of Trunk Id's | [required] |

### Return type

[**Vec<crate::models::TrunkMetrics>**](TrunkMetrics.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_telephony_providers_edges_trunkswithrecording

> crate::models::TrunkRecordingEnabledCount get_telephony_providers_edges_trunkswithrecording(trunk_type)
Get Counts of trunks that have recording disabled or enabled

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**trunk_type** | Option<**String**> | The type of this trunk base. |  |

### Return type

[**crate::models::TrunkRecordingEnabledCount**](TrunkRecordingEnabledCount.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_telephony_providers_edge_diagnostic_nslookup

> crate::models::EdgeNetworkDiagnostic post_telephony_providers_edge_diagnostic_nslookup(edge_id, body)
Nslookup request command to collect networking-related information from an Edge for a target IP or host.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edge_id** | **String** | Edge Id | [required] |
**body** | [**EdgeNetworkDiagnosticRequest**](EdgeNetworkDiagnosticRequest.md) | request payload to get network diagnostic | [required] |

### Return type

[**crate::models::EdgeNetworkDiagnostic**](EdgeNetworkDiagnostic.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_telephony_providers_edge_diagnostic_ping

> crate::models::EdgeNetworkDiagnostic post_telephony_providers_edge_diagnostic_ping(edge_id, body)
Ping Request command to collect networking-related information from an Edge for a target IP or host.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edge_id** | **String** | Edge Id | [required] |
**body** | [**EdgeNetworkDiagnosticRequest**](EdgeNetworkDiagnosticRequest.md) | request payload to get network diagnostic | [required] |

### Return type

[**crate::models::EdgeNetworkDiagnostic**](EdgeNetworkDiagnostic.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_telephony_providers_edge_diagnostic_route

> crate::models::EdgeNetworkDiagnostic post_telephony_providers_edge_diagnostic_route(edge_id, body)
Route request command to collect networking-related information from an Edge for a target IP or host.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edge_id** | **String** | Edge Id | [required] |
**body** | [**EdgeNetworkDiagnosticRequest**](EdgeNetworkDiagnosticRequest.md) | request payload to get network diagnostic | [required] |

### Return type

[**crate::models::EdgeNetworkDiagnostic**](EdgeNetworkDiagnostic.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_telephony_providers_edge_diagnostic_tracepath

> crate::models::EdgeNetworkDiagnostic post_telephony_providers_edge_diagnostic_tracepath(edge_id, body)
Tracepath request command to collect networking-related information from an Edge for a target IP or host.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edge_id** | **String** | Edge Id | [required] |
**body** | [**EdgeNetworkDiagnosticRequest**](EdgeNetworkDiagnosticRequest.md) | request payload to get network diagnostic | [required] |

### Return type

[**crate::models::EdgeNetworkDiagnostic**](EdgeNetworkDiagnostic.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_telephony_providers_edge_logicalinterfaces

> crate::models::DomainLogicalInterface post_telephony_providers_edge_logicalinterfaces(edge_id, body)
Create an edge logical interface.

Create

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edge_id** | **String** | Edge ID | [required] |
**body** | [**DomainLogicalInterface**](DomainLogicalInterface.md) | Logical interface | [required] |

### Return type

[**crate::models::DomainLogicalInterface**](DomainLogicalInterface.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_telephony_providers_edge_logs_job_upload

> post_telephony_providers_edge_logs_job_upload(edge_id, job_id, body)
Request that the specified fileIds be uploaded from the Edge.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edge_id** | **String** | Edge ID | [required] |
**job_id** | **String** | Job ID | [required] |
**body** | [**EdgeLogsJobUploadRequest**](EdgeLogsJobUploadRequest.md) | Log upload request | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_telephony_providers_edge_logs_jobs

> crate::models::EdgeLogsJobResponse post_telephony_providers_edge_logs_jobs(edge_id, body)
Create a job to upload a list of Edge logs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edge_id** | **String** | Edge ID | [required] |
**body** | [**EdgeLogsJobRequest**](EdgeLogsJobRequest.md) | EdgeLogsJobRequest | [required] |

### Return type

[**crate::models::EdgeLogsJobResponse**](EdgeLogsJobResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_telephony_providers_edge_reboot

> String post_telephony_providers_edge_reboot(edge_id, body)
Reboot an Edge

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edge_id** | **String** | Edge ID | [required] |
**body** | Option<[**EdgeRebootParameters**](EdgeRebootParameters.md)> | Parameters for the edge reboot |  |

### Return type

**String**

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_telephony_providers_edge_softwareupdate

> crate::models::DomainEdgeSoftwareUpdateDto post_telephony_providers_edge_softwareupdate(edge_id, body)
Starts a software update for this edge.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edge_id** | **String** | Edge ID | [required] |
**body** | [**DomainEdgeSoftwareUpdateDto**](DomainEdgeSoftwareUpdateDto.md) | Software update request | [required] |

### Return type

[**crate::models::DomainEdgeSoftwareUpdateDto**](DomainEdgeSoftwareUpdateDto.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_telephony_providers_edge_statuscode

> String post_telephony_providers_edge_statuscode(edge_id, body)
Take an Edge in or out of service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edge_id** | **String** | Edge ID | [required] |
**body** | Option<[**EdgeServiceStateRequest**](EdgeServiceStateRequest.md)> | Edge Service State |  |

### Return type

**String**

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_telephony_providers_edge_unpair

> String post_telephony_providers_edge_unpair(edge_id)
Unpair an Edge

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edge_id** | **String** | Edge Id | [required] |

### Return type

**String**

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_telephony_providers_edges

> crate::models::Edge post_telephony_providers_edges(body)
Create an edge.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**Edge**](Edge.md) | Edge | [required] |

### Return type

[**crate::models::Edge**](Edge.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_telephony_providers_edges_addressvalidation

> crate::models::ValidateAddressResponse post_telephony_providers_edges_addressvalidation(body)
Validates a street address

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ValidateAddressRequest**](ValidateAddressRequest.md) | Address | [required] |

### Return type

[**crate::models::ValidateAddressResponse**](ValidateAddressResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_telephony_providers_edges_certificateauthorities

> crate::models::DomainCertificateAuthority post_telephony_providers_edges_certificateauthorities(body)
Create a certificate authority.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**DomainCertificateAuthority**](DomainCertificateAuthority.md) | CertificateAuthority | [required] |

### Return type

[**crate::models::DomainCertificateAuthority**](DomainCertificateAuthority.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_telephony_providers_edges_didpools

> crate::models::DidPool post_telephony_providers_edges_didpools(body)
Create a new DID pool

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**DidPool**](DidPool.md) | DID pool | [required] |

### Return type

[**crate::models::DidPool**](DIDPool.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_telephony_providers_edges_edgegroups

> crate::models::EdgeGroup post_telephony_providers_edges_edgegroups(body)
Create an edge group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**EdgeGroup**](EdgeGroup.md) | EdgeGroup | [required] |

### Return type

[**crate::models::EdgeGroup**](EdgeGroup.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_telephony_providers_edges_extensionpools

> crate::models::ExtensionPool post_telephony_providers_edges_extensionpools(body)
Create a new extension pool

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ExtensionPool**](ExtensionPool.md) | ExtensionPool | [required] |

### Return type

[**crate::models::ExtensionPool**](ExtensionPool.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_telephony_providers_edges_outboundroutes

> crate::models::OutboundRoute post_telephony_providers_edges_outboundroutes(body)
Create outbound rule

This route is deprecated, use /telephony/providers/edges/sites/{siteId}/outboundroutes instead.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**OutboundRoute**](OutboundRoute.md) | OutboundRoute | [required] |

### Return type

[**crate::models::OutboundRoute**](OutboundRoute.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_telephony_providers_edges_phone_reboot

> post_telephony_providers_edges_phone_reboot(phone_id)
Reboot a Phone

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**phone_id** | **String** | Phone Id | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_telephony_providers_edges_phonebasesettings

> crate::models::PhoneBase post_telephony_providers_edges_phonebasesettings(body)
Create a new Phone Base Settings object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**PhoneBase**](PhoneBase.md) | Phone base settings | [required] |

### Return type

[**crate::models::PhoneBase**](PhoneBase.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_telephony_providers_edges_phones

> crate::models::Phone post_telephony_providers_edges_phones(body)
Create a new Phone

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**Phone**](Phone.md) | Phone | [required] |

### Return type

[**crate::models::Phone**](Phone.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_telephony_providers_edges_phones_reboot

> post_telephony_providers_edges_phones_reboot(body)
Reboot Multiple Phones

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**PhonesReboot**](PhonesReboot.md) | Phones | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_telephony_providers_edges_site_outboundroutes

> crate::models::OutboundRouteBase post_telephony_providers_edges_site_outboundroutes(site_id, body)
Create outbound route

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** | Site ID | [required] |
**body** | [**OutboundRouteBase**](OutboundRouteBase.md) | OutboundRoute | [required] |

### Return type

[**crate::models::OutboundRouteBase**](OutboundRouteBase.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_telephony_providers_edges_site_rebalance

> post_telephony_providers_edges_site_rebalance(site_id)
Triggers the rebalance operation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** | Site ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_telephony_providers_edges_sites

> crate::models::Site post_telephony_providers_edges_sites(body)
Create a Site.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**Site**](Site.md) | Site | [required] |

### Return type

[**crate::models::Site**](Site.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_telephony_providers_edges_trunkbasesettings

> crate::models::TrunkBase post_telephony_providers_edges_trunkbasesettings(body)
Create a Trunk Base Settings object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**TrunkBase**](TrunkBase.md) | Trunk base settings | [required] |

### Return type

[**crate::models::TrunkBase**](TrunkBase.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_telephony_providers_edge

> crate::models::Edge put_telephony_providers_edge(edge_id, body)
Update a edge.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edge_id** | **String** | Edge ID | [required] |
**body** | [**Edge**](Edge.md) | Edge | [required] |

### Return type

[**crate::models::Edge**](Edge.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_telephony_providers_edge_line

> crate::models::EdgeLine put_telephony_providers_edge_line(edge_id, line_id, body)
Update a line.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edge_id** | **String** | Edge ID | [required] |
**line_id** | **String** | Line ID | [required] |
**body** | [**EdgeLine**](EdgeLine.md) | Line | [required] |

### Return type

[**crate::models::EdgeLine**](EdgeLine.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_telephony_providers_edge_logicalinterface

> crate::models::DomainLogicalInterface put_telephony_providers_edge_logicalinterface(edge_id, interface_id, body)
Update an edge logical interface.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edge_id** | **String** | Edge ID | [required] |
**interface_id** | **String** | Interface ID | [required] |
**body** | [**DomainLogicalInterface**](DomainLogicalInterface.md) | Logical interface | [required] |

### Return type

[**crate::models::DomainLogicalInterface**](DomainLogicalInterface.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_telephony_providers_edges_certificateauthority

> crate::models::DomainCertificateAuthority put_telephony_providers_edges_certificateauthority(certificate_id, body)
Update a certificate authority.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**certificate_id** | **String** | Certificate ID | [required] |
**body** | [**DomainCertificateAuthority**](DomainCertificateAuthority.md) | Certificate authority | [required] |

### Return type

[**crate::models::DomainCertificateAuthority**](DomainCertificateAuthority.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_telephony_providers_edges_did

> crate::models::Did put_telephony_providers_edges_did(did_id, body)
Update a DID by ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did_id** | **String** | DID ID | [required] |
**body** | [**Did**](Did.md) | DID | [required] |

### Return type

[**crate::models::Did**](DID.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_telephony_providers_edges_didpool

> crate::models::DidPool put_telephony_providers_edges_didpool(did_pool_id, body)
Update a DID Pool by ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did_pool_id** | **String** | DID pool ID | [required] |
**body** | [**DidPool**](DidPool.md) | DID pool | [required] |

### Return type

[**crate::models::DidPool**](DIDPool.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_telephony_providers_edges_edgegroup

> crate::models::EdgeGroup put_telephony_providers_edges_edgegroup(edge_group_id, body)
Update an edge group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edge_group_id** | **String** | Edge group ID | [required] |
**body** | [**EdgeGroup**](EdgeGroup.md) | EdgeGroup | [required] |

### Return type

[**crate::models::EdgeGroup**](EdgeGroup.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_telephony_providers_edges_edgegroup_edgetrunkbase

> crate::models::EdgeTrunkBase put_telephony_providers_edges_edgegroup_edgetrunkbase(edgegroup_id, edgetrunkbase_id, body)
Update the edge trunk base associated with the edge group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edgegroup_id** | **String** | Edge Group ID | [required] |
**edgetrunkbase_id** | **String** | Edge Trunk Base ID | [required] |
**body** | [**EdgeTrunkBase**](EdgeTrunkBase.md) | EdgeTrunkBase | [required] |

### Return type

[**crate::models::EdgeTrunkBase**](EdgeTrunkBase.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_telephony_providers_edges_extension

> crate::models::Extension put_telephony_providers_edges_extension(extension_id, body)
Update an extension by ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**extension_id** | **String** | Extension ID | [required] |
**body** | [**Extension**](Extension.md) | Extension | [required] |

### Return type

[**crate::models::Extension**](Extension.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_telephony_providers_edges_extensionpool

> crate::models::ExtensionPool put_telephony_providers_edges_extensionpool(extension_pool_id, body)
Update an extension pool by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**extension_pool_id** | **String** | Extension pool ID | [required] |
**body** | [**ExtensionPool**](ExtensionPool.md) | ExtensionPool | [required] |

### Return type

[**crate::models::ExtensionPool**](ExtensionPool.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_telephony_providers_edges_outboundroute

> crate::models::OutboundRoute put_telephony_providers_edges_outboundroute(outbound_route_id, body)
Update outbound route

This route is deprecated, use /telephony/providers/edges/sites/{siteId}/outboundroutes/{outboundRouteId} instead.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**outbound_route_id** | **String** | Outbound route ID | [required] |
**body** | [**OutboundRoute**](OutboundRoute.md) | OutboundRoute | [required] |

### Return type

[**crate::models::OutboundRoute**](OutboundRoute.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_telephony_providers_edges_phone

> crate::models::Phone put_telephony_providers_edges_phone(phone_id, body)
Update a Phone by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**phone_id** | **String** | Phone ID | [required] |
**body** | [**Phone**](Phone.md) | Phone | [required] |

### Return type

[**crate::models::Phone**](Phone.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_telephony_providers_edges_phonebasesetting

> crate::models::PhoneBase put_telephony_providers_edges_phonebasesetting(phone_base_id, body)
Update a Phone Base Settings by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**phone_base_id** | **String** | Phone base ID | [required] |
**body** | [**PhoneBase**](PhoneBase.md) | Phone base settings | [required] |

### Return type

[**crate::models::PhoneBase**](PhoneBase.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_telephony_providers_edges_site

> crate::models::Site put_telephony_providers_edges_site(site_id, body)
Update a Site by ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** | Site ID | [required] |
**body** | [**Site**](Site.md) | Site | [required] |

### Return type

[**crate::models::Site**](Site.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_telephony_providers_edges_site_numberplans

> Vec<crate::models::NumberPlan> put_telephony_providers_edges_site_numberplans(site_id, body)
Update the list of Number Plans. A user can update maximum 200 number plans at a time.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** | Site ID | [required] |
**body** | [**Vec<crate::models::NumberPlan>**](NumberPlan.md) | List of number plans | [required] |

### Return type

[**Vec<crate::models::NumberPlan>**](NumberPlan.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_telephony_providers_edges_site_outboundroute

> crate::models::OutboundRouteBase put_telephony_providers_edges_site_outboundroute(site_id, outbound_route_id, body)
Update outbound route

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** | Site ID | [required] |
**outbound_route_id** | **String** | Outbound route ID | [required] |
**body** | [**OutboundRouteBase**](OutboundRouteBase.md) | OutboundRoute | [required] |

### Return type

[**crate::models::OutboundRouteBase**](OutboundRouteBase.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_telephony_providers_edges_trunkbasesetting

> crate::models::TrunkBase put_telephony_providers_edges_trunkbasesetting(trunk_base_settings_id, body)
Update a Trunk Base Settings object by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**trunk_base_settings_id** | **String** | Trunk Base ID | [required] |
**body** | [**TrunkBase**](TrunkBase.md) | Trunk base settings | [required] |

### Return type

[**crate::models::TrunkBase**](TrunkBase.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

