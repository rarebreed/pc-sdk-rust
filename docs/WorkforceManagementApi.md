# \WorkforceManagementApi

All URIs are relative to *https://api.mypurecloud.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_workforcemanagement_businessunit**](WorkforceManagementApi.md#delete_workforcemanagement_businessunit) | **DELETE** /api/v2/workforcemanagement/businessunits/{businessUnitId} | Delete business unit
[**delete_workforcemanagement_businessunit_activitycode**](WorkforceManagementApi.md#delete_workforcemanagement_businessunit_activitycode) | **DELETE** /api/v2/workforcemanagement/businessunits/{businessUnitId}/activitycodes/{activityCodeId} | Deletes an activity code
[**delete_workforcemanagement_businessunit_planninggroup**](WorkforceManagementApi.md#delete_workforcemanagement_businessunit_planninggroup) | **DELETE** /api/v2/workforcemanagement/businessunits/{businessUnitId}/planninggroups/{planningGroupId} | Deletes the planning group
[**delete_workforcemanagement_businessunit_scheduling_run**](WorkforceManagementApi.md#delete_workforcemanagement_businessunit_scheduling_run) | **DELETE** /api/v2/workforcemanagement/businessunits/{businessUnitId}/scheduling/runs/{runId} | Cancel a scheduling run
[**delete_workforcemanagement_businessunit_servicegoaltemplate**](WorkforceManagementApi.md#delete_workforcemanagement_businessunit_servicegoaltemplate) | **DELETE** /api/v2/workforcemanagement/businessunits/{businessUnitId}/servicegoaltemplates/{serviceGoalTemplateId} | Delete a service goal template
[**delete_workforcemanagement_businessunit_week_schedule**](WorkforceManagementApi.md#delete_workforcemanagement_businessunit_week_schedule) | **DELETE** /api/v2/workforcemanagement/businessunits/{businessUnitId}/weeks/{weekId}/schedules/{scheduleId} | Delete a schedule
[**delete_workforcemanagement_businessunit_week_shorttermforecast**](WorkforceManagementApi.md#delete_workforcemanagement_businessunit_week_shorttermforecast) | **DELETE** /api/v2/workforcemanagement/businessunits/{businessUnitId}/weeks/{weekDateId}/shorttermforecasts/{forecastId} | Delete a short term forecast
[**delete_workforcemanagement_calendar_url_ics**](WorkforceManagementApi.md#delete_workforcemanagement_calendar_url_ics) | **DELETE** /api/v2/workforcemanagement/calendar/url/ics | Disable generated calendar link for the current user
[**delete_workforcemanagement_managementunit**](WorkforceManagementApi.md#delete_workforcemanagement_managementunit) | **DELETE** /api/v2/workforcemanagement/managementunits/{managementUnitId} | Delete management unit
[**delete_workforcemanagement_managementunit_timeofflimit**](WorkforceManagementApi.md#delete_workforcemanagement_managementunit_timeofflimit) | **DELETE** /api/v2/workforcemanagement/managementunits/{managementUnitId}/timeofflimits/{timeOffLimitId} | Deletes a time off limit object
[**delete_workforcemanagement_managementunit_timeoffplan**](WorkforceManagementApi.md#delete_workforcemanagement_managementunit_timeoffplan) | **DELETE** /api/v2/workforcemanagement/managementunits/{managementUnitId}/timeoffplans/{timeOffPlanId} | Deletes a time off plan
[**delete_workforcemanagement_managementunit_workplan**](WorkforceManagementApi.md#delete_workforcemanagement_managementunit_workplan) | **DELETE** /api/v2/workforcemanagement/managementunits/{managementUnitId}/workplans/{workPlanId} | Delete a work plan
[**delete_workforcemanagement_managementunit_workplanrotation**](WorkforceManagementApi.md#delete_workforcemanagement_managementunit_workplanrotation) | **DELETE** /api/v2/workforcemanagement/managementunits/{managementUnitId}/workplanrotations/{workPlanRotationId} | Delete a work plan rotation
[**get_workforcemanagement_adherence**](WorkforceManagementApi.md#get_workforcemanagement_adherence) | **GET** /api/v2/workforcemanagement/adherence | Get a list of UserScheduleAdherence records for the requested users
[**get_workforcemanagement_adhocmodelingjob**](WorkforceManagementApi.md#get_workforcemanagement_adhocmodelingjob) | **GET** /api/v2/workforcemanagement/adhocmodelingjobs/{jobId} | Get status of the modeling job
[**get_workforcemanagement_agent_managementunit**](WorkforceManagementApi.md#get_workforcemanagement_agent_managementunit) | **GET** /api/v2/workforcemanagement/agents/{agentId}/managementunit | Get the management unit to which the agent belongs
[**get_workforcemanagement_agents_me_managementunit**](WorkforceManagementApi.md#get_workforcemanagement_agents_me_managementunit) | **GET** /api/v2/workforcemanagement/agents/me/managementunit | Get the management unit to which the currently logged in agent belongs
[**get_workforcemanagement_businessunit**](WorkforceManagementApi.md#get_workforcemanagement_businessunit) | **GET** /api/v2/workforcemanagement/businessunits/{businessUnitId} | Get business unit
[**get_workforcemanagement_businessunit_activitycode**](WorkforceManagementApi.md#get_workforcemanagement_businessunit_activitycode) | **GET** /api/v2/workforcemanagement/businessunits/{businessUnitId}/activitycodes/{activityCodeId} | Get an activity code
[**get_workforcemanagement_businessunit_activitycodes**](WorkforceManagementApi.md#get_workforcemanagement_businessunit_activitycodes) | **GET** /api/v2/workforcemanagement/businessunits/{businessUnitId}/activitycodes | Get activity codes
[**get_workforcemanagement_businessunit_intraday_planninggroups**](WorkforceManagementApi.md#get_workforcemanagement_businessunit_intraday_planninggroups) | **GET** /api/v2/workforcemanagement/businessunits/{businessUnitId}/intraday/planninggroups | Get intraday planning groups for the given date
[**get_workforcemanagement_businessunit_managementunits**](WorkforceManagementApi.md#get_workforcemanagement_businessunit_managementunits) | **GET** /api/v2/workforcemanagement/businessunits/{businessUnitId}/managementunits | Get all authorized management units in the business unit
[**get_workforcemanagement_businessunit_planninggroup**](WorkforceManagementApi.md#get_workforcemanagement_businessunit_planninggroup) | **GET** /api/v2/workforcemanagement/businessunits/{businessUnitId}/planninggroups/{planningGroupId} | Get a planning group
[**get_workforcemanagement_businessunit_planninggroups**](WorkforceManagementApi.md#get_workforcemanagement_businessunit_planninggroups) | **GET** /api/v2/workforcemanagement/businessunits/{businessUnitId}/planninggroups | Gets list of planning groups
[**get_workforcemanagement_businessunit_scheduling_run**](WorkforceManagementApi.md#get_workforcemanagement_businessunit_scheduling_run) | **GET** /api/v2/workforcemanagement/businessunits/{businessUnitId}/scheduling/runs/{runId} | Get a scheduling run
[**get_workforcemanagement_businessunit_scheduling_run_result**](WorkforceManagementApi.md#get_workforcemanagement_businessunit_scheduling_run_result) | **GET** /api/v2/workforcemanagement/businessunits/{businessUnitId}/scheduling/runs/{runId}/result | Get the result of a rescheduling operation
[**get_workforcemanagement_businessunit_scheduling_runs**](WorkforceManagementApi.md#get_workforcemanagement_businessunit_scheduling_runs) | **GET** /api/v2/workforcemanagement/businessunits/{businessUnitId}/scheduling/runs | Get the list of scheduling runs
[**get_workforcemanagement_businessunit_servicegoaltemplate**](WorkforceManagementApi.md#get_workforcemanagement_businessunit_servicegoaltemplate) | **GET** /api/v2/workforcemanagement/businessunits/{businessUnitId}/servicegoaltemplates/{serviceGoalTemplateId} | Get a service goal template
[**get_workforcemanagement_businessunit_servicegoaltemplates**](WorkforceManagementApi.md#get_workforcemanagement_businessunit_servicegoaltemplates) | **GET** /api/v2/workforcemanagement/businessunits/{businessUnitId}/servicegoaltemplates | Gets list of service goal templates
[**get_workforcemanagement_businessunit_week_schedule**](WorkforceManagementApi.md#get_workforcemanagement_businessunit_week_schedule) | **GET** /api/v2/workforcemanagement/businessunits/{businessUnitId}/weeks/{weekId}/schedules/{scheduleId} | Get the metadata for the schedule, describing which management units and agents are in the scheduleSchedule data can then be loaded with the query route
[**get_workforcemanagement_businessunit_week_schedule_generationresults**](WorkforceManagementApi.md#get_workforcemanagement_businessunit_week_schedule_generationresults) | **GET** /api/v2/workforcemanagement/businessunits/{businessUnitId}/weeks/{weekId}/schedules/{scheduleId}/generationresults | Get the generation results for a generated schedule
[**get_workforcemanagement_businessunit_week_schedule_headcountforecast**](WorkforceManagementApi.md#get_workforcemanagement_businessunit_week_schedule_headcountforecast) | **GET** /api/v2/workforcemanagement/businessunits/{businessUnitId}/weeks/{weekId}/schedules/{scheduleId}/headcountforecast | Get the headcount forecast by planning group for the schedule
[**get_workforcemanagement_businessunit_week_schedule_history_agent**](WorkforceManagementApi.md#get_workforcemanagement_businessunit_week_schedule_history_agent) | **GET** /api/v2/workforcemanagement/businessunits/{businessUnitId}/weeks/{weekId}/schedules/{scheduleId}/history/agents/{agentId} | Loads agent's schedule history.
[**get_workforcemanagement_businessunit_week_schedules**](WorkforceManagementApi.md#get_workforcemanagement_businessunit_week_schedules) | **GET** /api/v2/workforcemanagement/businessunits/{businessUnitId}/weeks/{weekId}/schedules | Get the list of week schedules for the specified week
[**get_workforcemanagement_businessunit_week_shorttermforecast**](WorkforceManagementApi.md#get_workforcemanagement_businessunit_week_shorttermforecast) | **GET** /api/v2/workforcemanagement/businessunits/{businessUnitId}/weeks/{weekDateId}/shorttermforecasts/{forecastId} | Get a short term forecast
[**get_workforcemanagement_businessunit_week_shorttermforecast_data**](WorkforceManagementApi.md#get_workforcemanagement_businessunit_week_shorttermforecast_data) | **GET** /api/v2/workforcemanagement/businessunits/{businessUnitId}/weeks/{weekDateId}/shorttermforecasts/{forecastId}/data | Get the result of a short term forecast calculation
[**get_workforcemanagement_businessunit_week_shorttermforecast_generationresults**](WorkforceManagementApi.md#get_workforcemanagement_businessunit_week_shorttermforecast_generationresults) | **GET** /api/v2/workforcemanagement/businessunits/{businessUnitId}/weeks/{weekDateId}/shorttermforecasts/{forecastId}/generationresults | Gets the forecast generation results
[**get_workforcemanagement_businessunit_week_shorttermforecast_longtermforecastdata**](WorkforceManagementApi.md#get_workforcemanagement_businessunit_week_shorttermforecast_longtermforecastdata) | **GET** /api/v2/workforcemanagement/businessunits/{businessUnitId}/weeks/{weekDateId}/shorttermforecasts/{forecastId}/longtermforecastdata | Get the result of a long term forecast calculation
[**get_workforcemanagement_businessunit_week_shorttermforecast_planninggroups**](WorkforceManagementApi.md#get_workforcemanagement_businessunit_week_shorttermforecast_planninggroups) | **GET** /api/v2/workforcemanagement/businessunits/{businessUnitId}/weeks/{weekDateId}/shorttermforecasts/{forecastId}/planninggroups | Gets the forecast planning group snapshot
[**get_workforcemanagement_businessunit_week_shorttermforecasts**](WorkforceManagementApi.md#get_workforcemanagement_businessunit_week_shorttermforecasts) | **GET** /api/v2/workforcemanagement/businessunits/{businessUnitId}/weeks/{weekDateId}/shorttermforecasts | Get short term forecasts
[**get_workforcemanagement_businessunits**](WorkforceManagementApi.md#get_workforcemanagement_businessunits) | **GET** /api/v2/workforcemanagement/businessunits | Get business units
[**get_workforcemanagement_businessunits_divisionviews**](WorkforceManagementApi.md#get_workforcemanagement_businessunits_divisionviews) | **GET** /api/v2/workforcemanagement/businessunits/divisionviews | Get business units across divisions
[**get_workforcemanagement_calendar_data_ics**](WorkforceManagementApi.md#get_workforcemanagement_calendar_data_ics) | **GET** /api/v2/workforcemanagement/calendar/data/ics | Get ics formatted calendar based on shareable link
[**get_workforcemanagement_calendar_url_ics**](WorkforceManagementApi.md#get_workforcemanagement_calendar_url_ics) | **GET** /api/v2/workforcemanagement/calendar/url/ics | Get existing calendar link for the current user
[**get_workforcemanagement_historicaldata_deletejob**](WorkforceManagementApi.md#get_workforcemanagement_historicaldata_deletejob) | **GET** /api/v2/workforcemanagement/historicaldata/deletejob | Retrieves delete job status for historical data imports of the organization
[**get_workforcemanagement_historicaldata_importstatus**](WorkforceManagementApi.md#get_workforcemanagement_historicaldata_importstatus) | **GET** /api/v2/workforcemanagement/historicaldata/importstatus | Retrieves status of the historical data imports of the organization
[**get_workforcemanagement_managementunit**](WorkforceManagementApi.md#get_workforcemanagement_managementunit) | **GET** /api/v2/workforcemanagement/managementunits/{managementUnitId} | Get management unit
[**get_workforcemanagement_managementunit_activitycodes**](WorkforceManagementApi.md#get_workforcemanagement_managementunit_activitycodes) | **GET** /api/v2/workforcemanagement/managementunits/{managementUnitId}/activitycodes | Get activity codes
[**get_workforcemanagement_managementunit_adherence**](WorkforceManagementApi.md#get_workforcemanagement_managementunit_adherence) | **GET** /api/v2/workforcemanagement/managementunits/{managementUnitId}/adherence | Get a list of user schedule adherence records for the requested management unit
[**get_workforcemanagement_managementunit_agent**](WorkforceManagementApi.md#get_workforcemanagement_managementunit_agent) | **GET** /api/v2/workforcemanagement/managementunits/{managementUnitId}/agents/{agentId} | Get data for agent in the management unit
[**get_workforcemanagement_managementunit_agent_shifttrades**](WorkforceManagementApi.md#get_workforcemanagement_managementunit_agent_shifttrades) | **GET** /api/v2/workforcemanagement/managementunits/{managementUnitId}/agents/{agentId}/shifttrades | Gets all the shift trades for a given agent
[**get_workforcemanagement_managementunit_shifttrades_matched**](WorkforceManagementApi.md#get_workforcemanagement_managementunit_shifttrades_matched) | **GET** /api/v2/workforcemanagement/managementunits/{managementUnitId}/shifttrades/matched | Gets a summary of all shift trades in the matched state
[**get_workforcemanagement_managementunit_shifttrades_users**](WorkforceManagementApi.md#get_workforcemanagement_managementunit_shifttrades_users) | **GET** /api/v2/workforcemanagement/managementunits/{managementUnitId}/shifttrades/users | Gets list of users available for whom you can send direct shift trade requests
[**get_workforcemanagement_managementunit_timeofflimit**](WorkforceManagementApi.md#get_workforcemanagement_managementunit_timeofflimit) | **GET** /api/v2/workforcemanagement/managementunits/{managementUnitId}/timeofflimits/{timeOffLimitId} | Gets a time off limit object
[**get_workforcemanagement_managementunit_timeofflimits**](WorkforceManagementApi.md#get_workforcemanagement_managementunit_timeofflimits) | **GET** /api/v2/workforcemanagement/managementunits/{managementUnitId}/timeofflimits | Gets a list of time off limit objects under management unit.
[**get_workforcemanagement_managementunit_timeoffplan**](WorkforceManagementApi.md#get_workforcemanagement_managementunit_timeoffplan) | **GET** /api/v2/workforcemanagement/managementunits/{managementUnitId}/timeoffplans/{timeOffPlanId} | Gets a time off plan
[**get_workforcemanagement_managementunit_timeoffplans**](WorkforceManagementApi.md#get_workforcemanagement_managementunit_timeoffplans) | **GET** /api/v2/workforcemanagement/managementunits/{managementUnitId}/timeoffplans | Gets a list of time off plans
[**get_workforcemanagement_managementunit_user_timeoffrequest**](WorkforceManagementApi.md#get_workforcemanagement_managementunit_user_timeoffrequest) | **GET** /api/v2/workforcemanagement/managementunits/{managementUnitId}/users/{userId}/timeoffrequests/{timeOffRequestId} | Get a time off request
[**get_workforcemanagement_managementunit_user_timeoffrequest_timeofflimits**](WorkforceManagementApi.md#get_workforcemanagement_managementunit_user_timeoffrequest_timeofflimits) | **GET** /api/v2/workforcemanagement/managementunits/{managementUnitId}/users/{userId}/timeoffrequests/{timeOffRequestId}/timeofflimits | Retrieves time off limit, allocated and waitlisted values according to specific time off request
[**get_workforcemanagement_managementunit_user_timeoffrequests**](WorkforceManagementApi.md#get_workforcemanagement_managementunit_user_timeoffrequests) | **GET** /api/v2/workforcemanagement/managementunits/{managementUnitId}/users/{userId}/timeoffrequests | Get a list of time off requests for a given user
[**get_workforcemanagement_managementunit_users**](WorkforceManagementApi.md#get_workforcemanagement_managementunit_users) | **GET** /api/v2/workforcemanagement/managementunits/{managementUnitId}/users | Get users in the management unit
[**get_workforcemanagement_managementunit_week_schedule**](WorkforceManagementApi.md#get_workforcemanagement_managementunit_week_schedule) | **GET** /api/v2/workforcemanagement/managementunits/{managementUnitId}/weeks/{weekId}/schedules/{scheduleId} | Deprecated.  Use the equivalent business unit resource instead. Get a week schedule
[**get_workforcemanagement_managementunit_week_schedules**](WorkforceManagementApi.md#get_workforcemanagement_managementunit_week_schedules) | **GET** /api/v2/workforcemanagement/managementunits/{managementUnitId}/weeks/{weekId}/schedules | Deprecated.  Use the equivalent business unit resource instead. Get the list of schedules in a week in management unit
[**get_workforcemanagement_managementunit_week_shifttrades**](WorkforceManagementApi.md#get_workforcemanagement_managementunit_week_shifttrades) | **GET** /api/v2/workforcemanagement/managementunits/{managementUnitId}/weeks/{weekDateId}/shifttrades | Gets all the shift trades for a given week
[**get_workforcemanagement_managementunit_workplan**](WorkforceManagementApi.md#get_workforcemanagement_managementunit_workplan) | **GET** /api/v2/workforcemanagement/managementunits/{managementUnitId}/workplans/{workPlanId} | Get a work plan
[**get_workforcemanagement_managementunit_workplanrotation**](WorkforceManagementApi.md#get_workforcemanagement_managementunit_workplanrotation) | **GET** /api/v2/workforcemanagement/managementunits/{managementUnitId}/workplanrotations/{workPlanRotationId} | Get a work plan rotation
[**get_workforcemanagement_managementunit_workplanrotations**](WorkforceManagementApi.md#get_workforcemanagement_managementunit_workplanrotations) | **GET** /api/v2/workforcemanagement/managementunits/{managementUnitId}/workplanrotations | Get work plan rotations
[**get_workforcemanagement_managementunit_workplans**](WorkforceManagementApi.md#get_workforcemanagement_managementunit_workplans) | **GET** /api/v2/workforcemanagement/managementunits/{managementUnitId}/workplans | Get work plans
[**get_workforcemanagement_managementunits**](WorkforceManagementApi.md#get_workforcemanagement_managementunits) | **GET** /api/v2/workforcemanagement/managementunits | Get management units
[**get_workforcemanagement_managementunits_divisionviews**](WorkforceManagementApi.md#get_workforcemanagement_managementunits_divisionviews) | **GET** /api/v2/workforcemanagement/managementunits/divisionviews | Get management units across divisions
[**get_workforcemanagement_notifications**](WorkforceManagementApi.md#get_workforcemanagement_notifications) | **GET** /api/v2/workforcemanagement/notifications | Get a list of notifications for the current user
[**get_workforcemanagement_schedulingjob**](WorkforceManagementApi.md#get_workforcemanagement_schedulingjob) | **GET** /api/v2/workforcemanagement/schedulingjobs/{jobId} | Get status of the scheduling job
[**get_workforcemanagement_shifttrades**](WorkforceManagementApi.md#get_workforcemanagement_shifttrades) | **GET** /api/v2/workforcemanagement/shifttrades | Gets all of my shift trades
[**get_workforcemanagement_timeoffrequest**](WorkforceManagementApi.md#get_workforcemanagement_timeoffrequest) | **GET** /api/v2/workforcemanagement/timeoffrequests/{timeOffRequestId} | Get a time off request for the current user
[**get_workforcemanagement_timeoffrequest_waitlistpositions**](WorkforceManagementApi.md#get_workforcemanagement_timeoffrequest_waitlistpositions) | **GET** /api/v2/workforcemanagement/timeoffrequests/{timeOffRequestId}/waitlistpositions | Get the daily waitlist positions of a time off request for the current user
[**get_workforcemanagement_timeoffrequests**](WorkforceManagementApi.md#get_workforcemanagement_timeoffrequests) | **GET** /api/v2/workforcemanagement/timeoffrequests | Get a list of time off requests for the current user
[**patch_workforcemanagement_businessunit**](WorkforceManagementApi.md#patch_workforcemanagement_businessunit) | **PATCH** /api/v2/workforcemanagement/businessunits/{businessUnitId} | Update business unit
[**patch_workforcemanagement_businessunit_activitycode**](WorkforceManagementApi.md#patch_workforcemanagement_businessunit_activitycode) | **PATCH** /api/v2/workforcemanagement/businessunits/{businessUnitId}/activitycodes/{activityCodeId} | Update an activity code
[**patch_workforcemanagement_businessunit_planninggroup**](WorkforceManagementApi.md#patch_workforcemanagement_businessunit_planninggroup) | **PATCH** /api/v2/workforcemanagement/businessunits/{businessUnitId}/planninggroups/{planningGroupId} | Updates the planning group
[**patch_workforcemanagement_businessunit_scheduling_run**](WorkforceManagementApi.md#patch_workforcemanagement_businessunit_scheduling_run) | **PATCH** /api/v2/workforcemanagement/businessunits/{businessUnitId}/scheduling/runs/{runId} | Mark a schedule run as applied
[**patch_workforcemanagement_businessunit_servicegoaltemplate**](WorkforceManagementApi.md#patch_workforcemanagement_businessunit_servicegoaltemplate) | **PATCH** /api/v2/workforcemanagement/businessunits/{businessUnitId}/servicegoaltemplates/{serviceGoalTemplateId} | Updates a service goal template
[**patch_workforcemanagement_managementunit**](WorkforceManagementApi.md#patch_workforcemanagement_managementunit) | **PATCH** /api/v2/workforcemanagement/managementunits/{managementUnitId} | Update the requested management unit
[**patch_workforcemanagement_managementunit_timeofflimit**](WorkforceManagementApi.md#patch_workforcemanagement_managementunit_timeofflimit) | **PATCH** /api/v2/workforcemanagement/managementunits/{managementUnitId}/timeofflimits/{timeOffLimitId} | Updates a time off limit object.
[**patch_workforcemanagement_managementunit_timeoffplan**](WorkforceManagementApi.md#patch_workforcemanagement_managementunit_timeoffplan) | **PATCH** /api/v2/workforcemanagement/managementunits/{managementUnitId}/timeoffplans/{timeOffPlanId} | Updates a time off plan
[**patch_workforcemanagement_managementunit_user_timeoffrequest**](WorkforceManagementApi.md#patch_workforcemanagement_managementunit_user_timeoffrequest) | **PATCH** /api/v2/workforcemanagement/managementunits/{managementUnitId}/users/{userId}/timeoffrequests/{timeOffRequestId} | Update a time off request
[**patch_workforcemanagement_managementunit_week_shifttrade**](WorkforceManagementApi.md#patch_workforcemanagement_managementunit_week_shifttrade) | **PATCH** /api/v2/workforcemanagement/managementunits/{managementUnitId}/weeks/{weekDateId}/shifttrades/{tradeId} | Updates a shift trade. This route can only be called by the initiating agent
[**patch_workforcemanagement_managementunit_workplan**](WorkforceManagementApi.md#patch_workforcemanagement_managementunit_workplan) | **PATCH** /api/v2/workforcemanagement/managementunits/{managementUnitId}/workplans/{workPlanId} | Update a work plan
[**patch_workforcemanagement_managementunit_workplanrotation**](WorkforceManagementApi.md#patch_workforcemanagement_managementunit_workplanrotation) | **PATCH** /api/v2/workforcemanagement/managementunits/{managementUnitId}/workplanrotations/{workPlanRotationId} | Update a work plan rotation
[**patch_workforcemanagement_timeoffrequest**](WorkforceManagementApi.md#patch_workforcemanagement_timeoffrequest) | **PATCH** /api/v2/workforcemanagement/timeoffrequests/{timeOffRequestId} | Update a time off request for the current user
[**post_workforcemanagement_adherence_historical**](WorkforceManagementApi.md#post_workforcemanagement_adherence_historical) | **POST** /api/v2/workforcemanagement/adherence/historical | Request a historical adherence report for users across management units
[**post_workforcemanagement_agentschedules_mine**](WorkforceManagementApi.md#post_workforcemanagement_agentschedules_mine) | **POST** /api/v2/workforcemanagement/agentschedules/mine | Get published schedule for the current user
[**post_workforcemanagement_businessunit_activitycodes**](WorkforceManagementApi.md#post_workforcemanagement_businessunit_activitycodes) | **POST** /api/v2/workforcemanagement/businessunits/{businessUnitId}/activitycodes | Create a new activity code
[**post_workforcemanagement_businessunit_agentschedules_search**](WorkforceManagementApi.md#post_workforcemanagement_businessunit_agentschedules_search) | **POST** /api/v2/workforcemanagement/businessunits/{businessUnitId}/agentschedules/search | Search published schedules
[**post_workforcemanagement_businessunit_intraday**](WorkforceManagementApi.md#post_workforcemanagement_businessunit_intraday) | **POST** /api/v2/workforcemanagement/businessunits/{businessUnitId}/intraday | Get intraday data for the given date for the requested planningGroupIds
[**post_workforcemanagement_businessunit_planninggroups**](WorkforceManagementApi.md#post_workforcemanagement_businessunit_planninggroups) | **POST** /api/v2/workforcemanagement/businessunits/{businessUnitId}/planninggroups | Adds a new planning group
[**post_workforcemanagement_businessunit_servicegoaltemplates**](WorkforceManagementApi.md#post_workforcemanagement_businessunit_servicegoaltemplates) | **POST** /api/v2/workforcemanagement/businessunits/{businessUnitId}/servicegoaltemplates | Adds a new service goal template
[**post_workforcemanagement_businessunit_week_schedule_agentschedules_query**](WorkforceManagementApi.md#post_workforcemanagement_businessunit_week_schedule_agentschedules_query) | **POST** /api/v2/workforcemanagement/businessunits/{businessUnitId}/weeks/{weekId}/schedules/{scheduleId}/agentschedules/query | Loads agent schedule data from the schedule. Used in combination with the metadata route
[**post_workforcemanagement_businessunit_week_schedule_copy**](WorkforceManagementApi.md#post_workforcemanagement_businessunit_week_schedule_copy) | **POST** /api/v2/workforcemanagement/businessunits/{businessUnitId}/weeks/{weekId}/schedules/{scheduleId}/copy | Copy a schedule
[**post_workforcemanagement_businessunit_week_schedule_reschedule**](WorkforceManagementApi.md#post_workforcemanagement_businessunit_week_schedule_reschedule) | **POST** /api/v2/workforcemanagement/businessunits/{businessUnitId}/weeks/{weekId}/schedules/{scheduleId}/reschedule | Start a rescheduling run
[**post_workforcemanagement_businessunit_week_schedules**](WorkforceManagementApi.md#post_workforcemanagement_businessunit_week_schedules) | **POST** /api/v2/workforcemanagement/businessunits/{businessUnitId}/weeks/{weekId}/schedules | Create a blank schedule
[**post_workforcemanagement_businessunit_week_schedules_generate**](WorkforceManagementApi.md#post_workforcemanagement_businessunit_week_schedules_generate) | **POST** /api/v2/workforcemanagement/businessunits/{businessUnitId}/weeks/{weekId}/schedules/generate | Generate a schedule
[**post_workforcemanagement_businessunit_week_shorttermforecast_copy**](WorkforceManagementApi.md#post_workforcemanagement_businessunit_week_shorttermforecast_copy) | **POST** /api/v2/workforcemanagement/businessunits/{businessUnitId}/weeks/{weekDateId}/shorttermforecasts/{forecastId}/copy | Copy a short term forecast
[**post_workforcemanagement_businessunit_week_shorttermforecasts_generate**](WorkforceManagementApi.md#post_workforcemanagement_businessunit_week_shorttermforecasts_generate) | **POST** /api/v2/workforcemanagement/businessunits/{businessUnitId}/weeks/{weekDateId}/shorttermforecasts/generate | Generate a short term forecast
[**post_workforcemanagement_businessunits**](WorkforceManagementApi.md#post_workforcemanagement_businessunits) | **POST** /api/v2/workforcemanagement/businessunits | Add a new business unit
[**post_workforcemanagement_calendar_url_ics**](WorkforceManagementApi.md#post_workforcemanagement_calendar_url_ics) | **POST** /api/v2/workforcemanagement/calendar/url/ics | Create a newly generated calendar link for the current user; if the current user has previously generated one, the generated link will be returned
[**post_workforcemanagement_historicaldata_deletejob**](WorkforceManagementApi.md#post_workforcemanagement_historicaldata_deletejob) | **POST** /api/v2/workforcemanagement/historicaldata/deletejob | Delete the entries of the historical data imports in the organization
[**post_workforcemanagement_historicaldata_validate**](WorkforceManagementApi.md#post_workforcemanagement_historicaldata_validate) | **POST** /api/v2/workforcemanagement/historicaldata/validate | Trigger validation process for historical import
[**post_workforcemanagement_managementunit_agentschedules_search**](WorkforceManagementApi.md#post_workforcemanagement_managementunit_agentschedules_search) | **POST** /api/v2/workforcemanagement/managementunits/{managementUnitId}/agentschedules/search | Query published schedules for given given time range for set of users
[**post_workforcemanagement_managementunit_historicaladherencequery**](WorkforceManagementApi.md#post_workforcemanagement_managementunit_historicaladherencequery) | **POST** /api/v2/workforcemanagement/managementunits/{managementUnitId}/historicaladherencequery | Request a historical adherence report
[**post_workforcemanagement_managementunit_move**](WorkforceManagementApi.md#post_workforcemanagement_managementunit_move) | **POST** /api/v2/workforcemanagement/managementunits/{managementUnitId}/move | Move the requested management unit to a new business unit
[**post_workforcemanagement_managementunit_schedules_search**](WorkforceManagementApi.md#post_workforcemanagement_managementunit_schedules_search) | **POST** /api/v2/workforcemanagement/managementunits/{managementUnitId}/schedules/search | Query published schedules for given given time range for set of users
[**post_workforcemanagement_managementunit_timeofflimits**](WorkforceManagementApi.md#post_workforcemanagement_managementunit_timeofflimits) | **POST** /api/v2/workforcemanagement/managementunits/{managementUnitId}/timeofflimits | Creates a new time off limit object under management unit.
[**post_workforcemanagement_managementunit_timeofflimits_values_query**](WorkforceManagementApi.md#post_workforcemanagement_managementunit_timeofflimits_values_query) | **POST** /api/v2/workforcemanagement/managementunits/{managementUnitId}/timeofflimits/values/query | Retrieves time off limit related values based on a given set of filters.
[**post_workforcemanagement_managementunit_timeoffplans**](WorkforceManagementApi.md#post_workforcemanagement_managementunit_timeoffplans) | **POST** /api/v2/workforcemanagement/managementunits/{managementUnitId}/timeoffplans | Creates a new time off plan
[**post_workforcemanagement_managementunit_timeoffrequests**](WorkforceManagementApi.md#post_workforcemanagement_managementunit_timeoffrequests) | **POST** /api/v2/workforcemanagement/managementunits/{managementUnitId}/timeoffrequests | Create a new time off request
[**post_workforcemanagement_managementunit_timeoffrequests_query**](WorkforceManagementApi.md#post_workforcemanagement_managementunit_timeoffrequests_query) | **POST** /api/v2/workforcemanagement/managementunits/{managementUnitId}/timeoffrequests/query | Fetches time off requests matching the conditions specified in the request body
[**post_workforcemanagement_managementunit_timeoffrequests_waitlistpositions_query**](WorkforceManagementApi.md#post_workforcemanagement_managementunit_timeoffrequests_waitlistpositions_query) | **POST** /api/v2/workforcemanagement/managementunits/{managementUnitId}/timeoffrequests/waitlistpositions/query | Retrieves daily waitlist position for a list of time off requests
[**post_workforcemanagement_managementunit_week_shifttrade_match**](WorkforceManagementApi.md#post_workforcemanagement_managementunit_week_shifttrade_match) | **POST** /api/v2/workforcemanagement/managementunits/{managementUnitId}/weeks/{weekDateId}/shifttrades/{tradeId}/match | Matches a shift trade. This route can only be called by the receiving agent
[**post_workforcemanagement_managementunit_week_shifttrades**](WorkforceManagementApi.md#post_workforcemanagement_managementunit_week_shifttrades) | **POST** /api/v2/workforcemanagement/managementunits/{managementUnitId}/weeks/{weekDateId}/shifttrades | Adds a shift trade
[**post_workforcemanagement_managementunit_week_shifttrades_search**](WorkforceManagementApi.md#post_workforcemanagement_managementunit_week_shifttrades_search) | **POST** /api/v2/workforcemanagement/managementunits/{managementUnitId}/weeks/{weekDateId}/shifttrades/search | Searches for potential shift trade matches for the current agent
[**post_workforcemanagement_managementunit_week_shifttrades_state_bulk**](WorkforceManagementApi.md#post_workforcemanagement_managementunit_week_shifttrades_state_bulk) | **POST** /api/v2/workforcemanagement/managementunits/{managementUnitId}/weeks/{weekDateId}/shifttrades/state/bulk | Updates the state of a batch of shift trades
[**post_workforcemanagement_managementunit_workplan_copy**](WorkforceManagementApi.md#post_workforcemanagement_managementunit_workplan_copy) | **POST** /api/v2/workforcemanagement/managementunits/{managementUnitId}/workplans/{workPlanId}/copy | Create a copy of work plan
[**post_workforcemanagement_managementunit_workplan_validate**](WorkforceManagementApi.md#post_workforcemanagement_managementunit_workplan_validate) | **POST** /api/v2/workforcemanagement/managementunits/{managementUnitId}/workplans/{workPlanId}/validate | Validate Work Plan
[**post_workforcemanagement_managementunit_workplanrotation_copy**](WorkforceManagementApi.md#post_workforcemanagement_managementunit_workplanrotation_copy) | **POST** /api/v2/workforcemanagement/managementunits/{managementUnitId}/workplanrotations/{workPlanRotationId}/copy | Create a copy of work plan rotation
[**post_workforcemanagement_managementunit_workplanrotations**](WorkforceManagementApi.md#post_workforcemanagement_managementunit_workplanrotations) | **POST** /api/v2/workforcemanagement/managementunits/{managementUnitId}/workplanrotations | Create a new work plan rotation
[**post_workforcemanagement_managementunit_workplans**](WorkforceManagementApi.md#post_workforcemanagement_managementunit_workplans) | **POST** /api/v2/workforcemanagement/managementunits/{managementUnitId}/workplans | Create a new work plan
[**post_workforcemanagement_managementunits**](WorkforceManagementApi.md#post_workforcemanagement_managementunits) | **POST** /api/v2/workforcemanagement/managementunits | Add a management unit
[**post_workforcemanagement_notifications_update**](WorkforceManagementApi.md#post_workforcemanagement_notifications_update) | **POST** /api/v2/workforcemanagement/notifications/update | Mark a list of notifications as read or unread
[**post_workforcemanagement_schedules**](WorkforceManagementApi.md#post_workforcemanagement_schedules) | **POST** /api/v2/workforcemanagement/schedules | Get published schedule for the current user
[**post_workforcemanagement_timeofflimits_available_query**](WorkforceManagementApi.md#post_workforcemanagement_timeofflimits_available_query) | **POST** /api/v2/workforcemanagement/timeofflimits/available/query | Queries available time off for the current user
[**post_workforcemanagement_timeoffrequests**](WorkforceManagementApi.md#post_workforcemanagement_timeoffrequests) | **POST** /api/v2/workforcemanagement/timeoffrequests | Create a time off request for the current user
[**put_workforcemanagement_managementunit_timeofflimit_values**](WorkforceManagementApi.md#put_workforcemanagement_managementunit_timeofflimit_values) | **PUT** /api/v2/workforcemanagement/managementunits/{managementUnitId}/timeofflimits/{timeOffLimitId}/values | Sets daily values for a date range of time off limit object



## delete_workforcemanagement_businessunit

> delete_workforcemanagement_businessunit(business_unit_id)
Delete business unit

A business unit cannot be deleted if it contains one or more management units

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**business_unit_id** | **String** | The ID of the business unit, or 'mine' for the business unit of the logged-in user. | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_workforcemanagement_businessunit_activitycode

> delete_workforcemanagement_businessunit_activitycode(business_unit_id, activity_code_id)
Deletes an activity code

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**business_unit_id** | **String** | The ID of the business unit, or 'mine' for the business unit of the logged-in user. | [required] |
**activity_code_id** | **String** | The ID of the activity code to delete | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_workforcemanagement_businessunit_planninggroup

> delete_workforcemanagement_businessunit_planninggroup(business_unit_id, planning_group_id)
Deletes the planning group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**business_unit_id** | **String** | The ID of the business unit. | [required] |
**planning_group_id** | **String** | The ID of a planning group to delete | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_workforcemanagement_businessunit_scheduling_run

> delete_workforcemanagement_businessunit_scheduling_run(business_unit_id, run_id)
Cancel a scheduling run

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**business_unit_id** | **String** | The ID of the business unit | [required] |
**run_id** | **String** | The ID of the schedule run | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_workforcemanagement_businessunit_servicegoaltemplate

> delete_workforcemanagement_businessunit_servicegoaltemplate(business_unit_id, service_goal_template_id)
Delete a service goal template

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**business_unit_id** | **String** | The ID of the business unit. | [required] |
**service_goal_template_id** | **String** | The ID of the service goal template to delete | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_workforcemanagement_businessunit_week_schedule

> crate::models::BuAsyncScheduleResponse delete_workforcemanagement_businessunit_week_schedule(business_unit_id, week_id, schedule_id)
Delete a schedule

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**business_unit_id** | **String** | The ID of the business unit | [required] |
**week_id** | **String** | First day of schedule week in yyyy-MM-dd format. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd | [required] |
**schedule_id** | **String** | The ID of the schedule | [required] |

### Return type

[**crate::models::BuAsyncScheduleResponse**](BuAsyncScheduleResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_workforcemanagement_businessunit_week_shorttermforecast

> delete_workforcemanagement_businessunit_week_shorttermforecast(business_unit_id, week_date_id, forecast_id)
Delete a short term forecast

Must not be tied to any schedules

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**business_unit_id** | **String** | The business unit ID of the business unit to which the forecast belongs | [required] |
**week_date_id** | **String** | The week start date of the forecast in yyyy-MM-dd format. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd | [required] |
**forecast_id** | **String** | The ID of the forecast | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_workforcemanagement_calendar_url_ics

> delete_workforcemanagement_calendar_url_ics()
Disable generated calendar link for the current user

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_workforcemanagement_managementunit

> delete_workforcemanagement_managementunit(management_unit_id)
Delete management unit

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**management_unit_id** | **String** | The ID of the management unit, or 'mine' for the management unit of the logged-in user. | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_workforcemanagement_managementunit_timeofflimit

> delete_workforcemanagement_managementunit_timeofflimit(management_unit_id, time_off_limit_id)
Deletes a time off limit object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**management_unit_id** | **String** | The management unit ID of the management unit. | [required] |
**time_off_limit_id** | **String** | The ID of the time off limit object to delete | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_workforcemanagement_managementunit_timeoffplan

> delete_workforcemanagement_managementunit_timeoffplan(management_unit_id, time_off_plan_id)
Deletes a time off plan

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**management_unit_id** | **String** | The management unit ID. | [required] |
**time_off_plan_id** | **String** | The ID of the time off plan to delete | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_workforcemanagement_managementunit_workplan

> delete_workforcemanagement_managementunit_workplan(management_unit_id, work_plan_id)
Delete a work plan

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**management_unit_id** | **String** | The ID of the management unit, or 'mine' for the management unit of the logged-in user. | [required] |
**work_plan_id** | **String** | The ID of the work plan to delete | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_workforcemanagement_managementunit_workplanrotation

> delete_workforcemanagement_managementunit_workplanrotation(management_unit_id, work_plan_rotation_id)
Delete a work plan rotation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**management_unit_id** | **String** | The ID of the management unit, or 'mine' for the management unit of the logged-in user. | [required] |
**work_plan_rotation_id** | **String** | The ID of the work plan rotation to be deleted | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workforcemanagement_adherence

> Vec<crate::models::UserScheduleAdherence> get_workforcemanagement_adherence(user_id)
Get a list of UserScheduleAdherence records for the requested users

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | [**Vec<String>**](String.md) | User Id(s) for which to fetch current schedule adherence information.  Min 1, Max of 100 userIds per request | [required] |

### Return type

[**Vec<crate::models::UserScheduleAdherence>**](UserScheduleAdherence.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workforcemanagement_adhocmodelingjob

> crate::models::ModelingStatusResponse get_workforcemanagement_adhocmodelingjob(job_id)
Get status of the modeling job

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | The id of the modeling job | [required] |

### Return type

[**crate::models::ModelingStatusResponse**](ModelingStatusResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workforcemanagement_agent_managementunit

> crate::models::AgentManagementUnitReference get_workforcemanagement_agent_managementunit(agent_id)
Get the management unit to which the agent belongs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**agent_id** | **String** | The ID of the agent to look up | [required] |

### Return type

[**crate::models::AgentManagementUnitReference**](AgentManagementUnitReference.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workforcemanagement_agents_me_managementunit

> crate::models::AgentManagementUnitReference get_workforcemanagement_agents_me_managementunit()
Get the management unit to which the currently logged in agent belongs

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::AgentManagementUnitReference**](AgentManagementUnitReference.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workforcemanagement_businessunit

> crate::models::BusinessUnit get_workforcemanagement_businessunit(business_unit_id, expand)
Get business unit

Expanding \"settings\" will retrieve all settings.  All other expands will retrieve only the requested settings field(s).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**business_unit_id** | **String** | The ID of the business unit, or 'mine' for the business unit of the logged-in user. | [required] |
**expand** | Option<[**Vec<String>**](String.md)> |  |  |

### Return type

[**crate::models::BusinessUnit**](BusinessUnit.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workforcemanagement_businessunit_activitycode

> crate::models::BusinessUnitActivityCode get_workforcemanagement_businessunit_activitycode(business_unit_id, activity_code_id)
Get an activity code

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**business_unit_id** | **String** | The ID of the business unit, or 'mine' for the business unit of the logged-in user. | [required] |
**activity_code_id** | **String** | The ID of the activity code to fetch | [required] |

### Return type

[**crate::models::BusinessUnitActivityCode**](BusinessUnitActivityCode.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workforcemanagement_businessunit_activitycodes

> crate::models::BusinessUnitActivityCodeListing get_workforcemanagement_businessunit_activitycodes(business_unit_id)
Get activity codes

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**business_unit_id** | **String** | The ID of the business unit, or 'mine' for the business unit of the logged-in user. | [required] |

### Return type

[**crate::models::BusinessUnitActivityCodeListing**](BusinessUnitActivityCodeListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workforcemanagement_businessunit_intraday_planninggroups

> crate::models::WfmIntradayPlanningGroupListing get_workforcemanagement_businessunit_intraday_planninggroups(business_unit_id, date)
Get intraday planning groups for the given date

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**business_unit_id** | **String** | The business unit ID of the business unit | [required] |
**date** | **String** | yyyy-MM-dd date string interpreted in the configured business unit time zone. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd | [required] |

### Return type

[**crate::models::WfmIntradayPlanningGroupListing**](WfmIntradayPlanningGroupListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workforcemanagement_businessunit_managementunits

> crate::models::ManagementUnitListing get_workforcemanagement_businessunit_managementunits(business_unit_id, feature, division_id)
Get all authorized management units in the business unit

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**business_unit_id** | **String** | The ID of the business unit, or 'mine' for the business unit of the logged-in user. | [required] |
**feature** | Option<**String**> |  |  |
**division_id** | Option<**String**> |  |  |

### Return type

[**crate::models::ManagementUnitListing**](ManagementUnitListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workforcemanagement_businessunit_planninggroup

> crate::models::PlanningGroup get_workforcemanagement_businessunit_planninggroup(business_unit_id, planning_group_id)
Get a planning group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**business_unit_id** | **String** | The ID of the business unit. | [required] |
**planning_group_id** | **String** | The ID of a planning group to fetch | [required] |

### Return type

[**crate::models::PlanningGroup**](PlanningGroup.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workforcemanagement_businessunit_planninggroups

> crate::models::PlanningGroupList get_workforcemanagement_businessunit_planninggroups(business_unit_id)
Gets list of planning groups

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**business_unit_id** | **String** | The ID of the business unit. | [required] |

### Return type

[**crate::models::PlanningGroupList**](PlanningGroupList.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workforcemanagement_businessunit_scheduling_run

> crate::models::BuScheduleRun get_workforcemanagement_businessunit_scheduling_run(business_unit_id, run_id)
Get a scheduling run

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**business_unit_id** | **String** | The ID of the business unit | [required] |
**run_id** | **String** | The ID of the schedule run | [required] |

### Return type

[**crate::models::BuScheduleRun**](BuScheduleRun.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workforcemanagement_businessunit_scheduling_run_result

> crate::models::BuRescheduleResult get_workforcemanagement_businessunit_scheduling_run_result(business_unit_id, run_id, management_unit_ids, expand)
Get the result of a rescheduling operation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**business_unit_id** | **String** | The ID of the business unit | [required] |
**run_id** | **String** | The ID of the schedule run | [required] |
**management_unit_ids** | [**Vec<String>**](String.md) | The IDs of the management units for which to fetch the reschedule results | [required] |
**expand** | [**Vec<String>**](String.md) | The fields to expand. Omitting will return an empty response | [required] |

### Return type

[**crate::models::BuRescheduleResult**](BuRescheduleResult.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workforcemanagement_businessunit_scheduling_runs

> crate::models::BuScheduleRunListing get_workforcemanagement_businessunit_scheduling_runs(business_unit_id)
Get the list of scheduling runs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**business_unit_id** | **String** | The ID of the business unit | [required] |

### Return type

[**crate::models::BuScheduleRunListing**](BuScheduleRunListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workforcemanagement_businessunit_servicegoaltemplate

> crate::models::ServiceGoalTemplate get_workforcemanagement_businessunit_servicegoaltemplate(business_unit_id, service_goal_template_id)
Get a service goal template

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**business_unit_id** | **String** | The ID of the business unit. | [required] |
**service_goal_template_id** | **String** | The ID of a service goal template to fetch | [required] |

### Return type

[**crate::models::ServiceGoalTemplate**](ServiceGoalTemplate.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workforcemanagement_businessunit_servicegoaltemplates

> crate::models::ServiceGoalTemplateList get_workforcemanagement_businessunit_servicegoaltemplates(business_unit_id)
Gets list of service goal templates

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**business_unit_id** | **String** | The ID of the business unit. | [required] |

### Return type

[**crate::models::ServiceGoalTemplateList**](ServiceGoalTemplateList.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workforcemanagement_businessunit_week_schedule

> crate::models::BuScheduleMetadata get_workforcemanagement_businessunit_week_schedule(business_unit_id, week_id, schedule_id, expand)
Get the metadata for the schedule, describing which management units and agents are in the scheduleSchedule data can then be loaded with the query route

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**business_unit_id** | **String** | The ID of the business unit | [required] |
**week_id** | **String** | First day of schedule week in yyyy-MM-dd format. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd | [required] |
**schedule_id** | **String** | The ID of the schedule | [required] |
**expand** | Option<**String**> | expand |  |

### Return type

[**crate::models::BuScheduleMetadata**](BuScheduleMetadata.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workforcemanagement_businessunit_week_schedule_generationresults

> crate::models::ScheduleGenerationResult get_workforcemanagement_businessunit_week_schedule_generationresults(business_unit_id, week_id, schedule_id)
Get the generation results for a generated schedule

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**business_unit_id** | **String** | The ID of the business unit | [required] |
**week_id** | **String** | First day of schedule week in yyyy-MM-dd format. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd | [required] |
**schedule_id** | **String** | The ID of the schedule | [required] |

### Return type

[**crate::models::ScheduleGenerationResult**](ScheduleGenerationResult.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workforcemanagement_businessunit_week_schedule_headcountforecast

> crate::models::BuHeadcountForecastResponse get_workforcemanagement_businessunit_week_schedule_headcountforecast(business_unit_id, week_id, schedule_id, force_download)
Get the headcount forecast by planning group for the schedule

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**business_unit_id** | **String** | The ID of the business unit | [required] |
**week_id** | **String** | First day of schedule week in yyyy-MM-dd format. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd | [required] |
**schedule_id** | **String** | The ID of the schedule | [required] |
**force_download** | Option<**bool**> | Whether to force the result to come via download url.  For testing purposes only |  |

### Return type

[**crate::models::BuHeadcountForecastResponse**](BuHeadcountForecastResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workforcemanagement_businessunit_week_schedule_history_agent

> crate::models::BuAgentScheduleHistoryResponse get_workforcemanagement_businessunit_week_schedule_history_agent(business_unit_id, week_id, schedule_id, agent_id)
Loads agent's schedule history.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**business_unit_id** | **String** | The ID of the business unit | [required] |
**week_id** | **String** | First day of schedule week in yyyy-MM-dd format. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd | [required] |
**schedule_id** | **String** | The ID of the schedule | [required] |
**agent_id** | **String** | THe ID of the agent | [required] |

### Return type

[**crate::models::BuAgentScheduleHistoryResponse**](BuAgentScheduleHistoryResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workforcemanagement_businessunit_week_schedules

> crate::models::BuScheduleListing get_workforcemanagement_businessunit_week_schedules(business_unit_id, week_id, include_only_published, expand)
Get the list of week schedules for the specified week

Use \"recent\" (without quotes) for the `weekId` path parameter to fetch all forecasts for +/- 26 weeks from the current date. Response will include any schedule which spans the specified week

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**business_unit_id** | **String** | The ID of the business unit | [required] |
**week_id** | **String** | First day of schedule week in yyyy-MM-dd format, or 'recent' (without quotes) to get recent schedules | [required] |
**include_only_published** | Option<**bool**> | includeOnlyPublished |  |
**expand** | Option<**String**> | expand |  |

### Return type

[**crate::models::BuScheduleListing**](BuScheduleListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workforcemanagement_businessunit_week_shorttermforecast

> crate::models::BuShortTermForecast get_workforcemanagement_businessunit_week_shorttermforecast(business_unit_id, week_date_id, forecast_id, expand)
Get a short term forecast

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**business_unit_id** | **String** | The business unit ID of the business unit to which the forecast belongs | [required] |
**week_date_id** | **String** | The week start date of the forecast in yyyy-MM-dd format. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd | [required] |
**forecast_id** | **String** | The ID of the forecast | [required] |
**expand** | Option<[**Vec<String>**](String.md)> |  |  |

### Return type

[**crate::models::BuShortTermForecast**](BuShortTermForecast.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workforcemanagement_businessunit_week_shorttermforecast_data

> crate::models::BuForecastResultResponse get_workforcemanagement_businessunit_week_shorttermforecast_data(business_unit_id, week_date_id, forecast_id, week_number, force_download_service)
Get the result of a short term forecast calculation

Includes modifications unless you pass the doNotApplyModifications query parameter

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**business_unit_id** | **String** | The business unit ID of the business unit to which the forecast belongs | [required] |
**week_date_id** | **String** | The week start date of the forecast in yyyy-MM-dd format. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd | [required] |
**forecast_id** | **String** | The ID of the forecast | [required] |
**week_number** | Option<**i32**> | The week number to fetch (for multi-week forecasts) |  |
**force_download_service** | Option<**bool**> | Force the result of this operation to be sent via download service.  For testing/app development purposes |  |

### Return type

[**crate::models::BuForecastResultResponse**](BuForecastResultResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workforcemanagement_businessunit_week_shorttermforecast_generationresults

> crate::models::BuForecastGenerationResult get_workforcemanagement_businessunit_week_shorttermforecast_generationresults(business_unit_id, week_date_id, forecast_id)
Gets the forecast generation results

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**business_unit_id** | **String** | The business unit ID of the business unit to which the forecast belongs | [required] |
**week_date_id** | **String** | The week start date of the forecast in yyyy-MM-dd format. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd | [required] |
**forecast_id** | **String** | The ID of the forecast | [required] |

### Return type

[**crate::models::BuForecastGenerationResult**](BuForecastGenerationResult.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workforcemanagement_businessunit_week_shorttermforecast_longtermforecastdata

> crate::models::LongTermForecastResultResponse get_workforcemanagement_businessunit_week_shorttermforecast_longtermforecastdata(business_unit_id, week_date_id, forecast_id, force_download_service)
Get the result of a long term forecast calculation

Includes modifications unless you pass the doNotApplyModifications query parameter

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**business_unit_id** | **String** | The business unit ID of the business unit to which the forecast belongs | [required] |
**week_date_id** | **String** | The week start date of the forecast in yyyy-MM-dd format. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd | [required] |
**forecast_id** | **String** | The ID of the forecast | [required] |
**force_download_service** | Option<**bool**> | Force the result of this operation to be sent via download service.  For testing/app development purposes |  |

### Return type

[**crate::models::LongTermForecastResultResponse**](LongTermForecastResultResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workforcemanagement_businessunit_week_shorttermforecast_planninggroups

> crate::models::ForecastPlanningGroupsResponse get_workforcemanagement_businessunit_week_shorttermforecast_planninggroups(business_unit_id, week_date_id, forecast_id)
Gets the forecast planning group snapshot

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**business_unit_id** | **String** | The business unit ID of the business unit to which the forecast belongs | [required] |
**week_date_id** | **String** | The week start date of the forecast in yyyy-MM-dd format. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd | [required] |
**forecast_id** | **String** | The ID of the forecast | [required] |

### Return type

[**crate::models::ForecastPlanningGroupsResponse**](ForecastPlanningGroupsResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workforcemanagement_businessunit_week_shorttermforecasts

> crate::models::BuShortTermForecastListing get_workforcemanagement_businessunit_week_shorttermforecasts(business_unit_id, week_date_id)
Get short term forecasts

Use \"recent\" (without quotes) for the `weekDateId` path parameter to fetch all forecasts for +/- 26 weeks from the current date. Response will include any forecast which spans the specified week

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**business_unit_id** | **String** | The business unit ID of the business unit to which the forecast belongs | [required] |
**week_date_id** | **String** | The week start date of the forecast in yyyy-MM-dd format or 'recent' (without quotes) to fetch recent forecasts | [required] |

### Return type

[**crate::models::BuShortTermForecastListing**](BuShortTermForecastListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workforcemanagement_businessunits

> crate::models::BusinessUnitListing get_workforcemanagement_businessunits(feature, division_id)
Get business units

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**feature** | Option<**String**> |  |  |
**division_id** | Option<**String**> |  |  |

### Return type

[**crate::models::BusinessUnitListing**](BusinessUnitListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workforcemanagement_businessunits_divisionviews

> crate::models::BusinessUnitListing get_workforcemanagement_businessunits_divisionviews(division_id)
Get business units across divisions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**division_id** | Option<[**Vec<String>**](String.md)> | The divisionIds to filter by. If omitted, will return business units in all divisions |  |

### Return type

[**crate::models::BusinessUnitListing**](BusinessUnitListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workforcemanagement_calendar_data_ics

> String get_workforcemanagement_calendar_data_ics(calendar_id)
Get ics formatted calendar based on shareable link

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**calendar_id** | **String** | The id of the ics-formatted calendar | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/calendar

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workforcemanagement_calendar_url_ics

> crate::models::CalendarUrlResponse get_workforcemanagement_calendar_url_ics()
Get existing calendar link for the current user

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CalendarUrlResponse**](CalendarUrlResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workforcemanagement_historicaldata_deletejob

> crate::models::HistoricalImportDeleteJobResponse get_workforcemanagement_historicaldata_deletejob()
Retrieves delete job status for historical data imports of the organization

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::HistoricalImportDeleteJobResponse**](HistoricalImportDeleteJobResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workforcemanagement_historicaldata_importstatus

> crate::models::HistoricalImportStatusListing get_workforcemanagement_historicaldata_importstatus()
Retrieves status of the historical data imports of the organization

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::HistoricalImportStatusListing**](HistoricalImportStatusListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workforcemanagement_managementunit

> crate::models::ManagementUnit get_workforcemanagement_managementunit(management_unit_id, expand)
Get management unit

settings.shortTermForecasting is deprecated and now lives on the business unit

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**management_unit_id** | **String** | The ID of the management unit, or 'mine' for the management unit of the logged-in user. | [required] |
**expand** | Option<[**Vec<String>**](String.md)> |  |  |

### Return type

[**crate::models::ManagementUnit**](ManagementUnit.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workforcemanagement_managementunit_activitycodes

> crate::models::ActivityCodeContainer get_workforcemanagement_managementunit_activitycodes(management_unit_id)
Get activity codes

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**management_unit_id** | **String** | The ID of the management unit, or 'mine' for the management unit of the logged-in user. | [required] |

### Return type

[**crate::models::ActivityCodeContainer**](ActivityCodeContainer.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workforcemanagement_managementunit_adherence

> crate::models::UserScheduleAdherenceListing get_workforcemanagement_managementunit_adherence(management_unit_id, force_download_service)
Get a list of user schedule adherence records for the requested management unit

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**management_unit_id** | **String** | The management unit ID of the management unit. | [required] |
**force_download_service** | Option<**bool**> | Force the result of this operation to be sent via download service.  For testing/app development purposes |  |

### Return type

[**crate::models::UserScheduleAdherenceListing**](UserScheduleAdherenceListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workforcemanagement_managementunit_agent

> crate::models::WfmAgent get_workforcemanagement_managementunit_agent(management_unit_id, agent_id, exclude_capabilities)
Get data for agent in the management unit

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**management_unit_id** | **String** | The id of the management unit, or 'mine' for the management unit of the logged-in user. | [required] |
**agent_id** | **String** | The agent id | [required] |
**exclude_capabilities** | Option<**bool**> | Excludes all capabilities of the agent such as queues, languages, and skills |  |

### Return type

[**crate::models::WfmAgent**](WfmAgent.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workforcemanagement_managementunit_agent_shifttrades

> crate::models::ShiftTradeListResponse get_workforcemanagement_managementunit_agent_shifttrades(management_unit_id, agent_id)
Gets all the shift trades for a given agent

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**management_unit_id** | **String** | The id of the management unit, or 'mine' for the management unit of the logged-in user. | [required] |
**agent_id** | **String** | The agent id | [required] |

### Return type

[**crate::models::ShiftTradeListResponse**](ShiftTradeListResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workforcemanagement_managementunit_shifttrades_matched

> crate::models::ShiftTradeMatchesSummaryResponse get_workforcemanagement_managementunit_shifttrades_matched(management_unit_id)
Gets a summary of all shift trades in the matched state

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**management_unit_id** | **String** | The management unit ID of the management unit, or 'mine' for the management unit of the logged-in user. | [required] |

### Return type

[**crate::models::ShiftTradeMatchesSummaryResponse**](ShiftTradeMatchesSummaryResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workforcemanagement_managementunit_shifttrades_users

> crate::models::WfmUserEntityListing get_workforcemanagement_managementunit_shifttrades_users(management_unit_id)
Gets list of users available for whom you can send direct shift trade requests

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**management_unit_id** | **String** | The management unit ID of the management unit, or 'mine' for the management unit of the logged-in user. | [required] |

### Return type

[**crate::models::WfmUserEntityListing**](WfmUserEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workforcemanagement_managementunit_timeofflimit

> crate::models::TimeOffLimit get_workforcemanagement_managementunit_timeofflimit(management_unit_id, time_off_limit_id)
Gets a time off limit object

Returns properties of time off limit object, but not daily values.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**management_unit_id** | **String** | The management unit ID of the management unit. | [required] |
**time_off_limit_id** | **String** | The ID of the time off limit to fetch | [required] |

### Return type

[**crate::models::TimeOffLimit**](TimeOffLimit.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workforcemanagement_managementunit_timeofflimits

> crate::models::TimeOffLimitListing get_workforcemanagement_managementunit_timeofflimits(management_unit_id)
Gets a list of time off limit objects under management unit.

Currently only one time off limit object is allowed under management unit, so the list contains either 0 or 1 element.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**management_unit_id** | **String** | The management unit ID of the management unit. | [required] |

### Return type

[**crate::models::TimeOffLimitListing**](TimeOffLimitListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workforcemanagement_managementunit_timeoffplan

> crate::models::TimeOffPlan get_workforcemanagement_managementunit_timeoffplan(management_unit_id, time_off_plan_id)
Gets a time off plan

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**management_unit_id** | **String** | The management unit ID. | [required] |
**time_off_plan_id** | **String** | The ID of the time off plan to fetch | [required] |

### Return type

[**crate::models::TimeOffPlan**](TimeOffPlan.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workforcemanagement_managementunit_timeoffplans

> crate::models::TimeOffPlanListing get_workforcemanagement_managementunit_timeoffplans(management_unit_id)
Gets a list of time off plans

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**management_unit_id** | **String** | The management unit ID. | [required] |

### Return type

[**crate::models::TimeOffPlanListing**](TimeOffPlanListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workforcemanagement_managementunit_user_timeoffrequest

> crate::models::TimeOffRequestResponse get_workforcemanagement_managementunit_user_timeoffrequest(management_unit_id, user_id, time_off_request_id)
Get a time off request

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**management_unit_id** | **String** | The muId of the management unit, or 'mine' for the management unit of the logged-in user. | [required] |
**user_id** | **String** | The userId to whom the Time Off Request applies. | [required] |
**time_off_request_id** | **String** | Time Off Request Id | [required] |

### Return type

[**crate::models::TimeOffRequestResponse**](TimeOffRequestResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workforcemanagement_managementunit_user_timeoffrequest_timeofflimits

> crate::models::QueryTimeOffLimitValuesResponse get_workforcemanagement_managementunit_user_timeoffrequest_timeofflimits(management_unit_id, user_id, time_off_request_id)
Retrieves time off limit, allocated and waitlisted values according to specific time off request

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**management_unit_id** | **String** | The management unit ID of the management unit. | [required] |
**user_id** | **String** | The userId to whom the time off request applies. | [required] |
**time_off_request_id** | **String** | The ID of the time off request, which dates and activityCodeId determine limit values to retrieve | [required] |

### Return type

[**crate::models::QueryTimeOffLimitValuesResponse**](QueryTimeOffLimitValuesResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workforcemanagement_managementunit_user_timeoffrequests

> crate::models::TimeOffRequestList get_workforcemanagement_managementunit_user_timeoffrequests(management_unit_id, user_id, recently_reviewed)
Get a list of time off requests for a given user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**management_unit_id** | **String** | The muId of the management unit, or 'mine' for the management unit of the logged-in user. | [required] |
**user_id** | **String** | The userId to whom the Time Off Request applies. | [required] |
**recently_reviewed** | Option<**bool**> | Limit results to requests that have been reviewed within the preceding 30 days |  |[default to false]

### Return type

[**crate::models::TimeOffRequestList**](TimeOffRequestList.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workforcemanagement_managementunit_users

> crate::models::WfmUserEntityListing get_workforcemanagement_managementunit_users(management_unit_id)
Get users in the management unit

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**management_unit_id** | **String** | The management unit ID of the management unit, or 'mine' for the management unit of the logged-in user. | [required] |

### Return type

[**crate::models::WfmUserEntityListing**](WfmUserEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workforcemanagement_managementunit_week_schedule

> crate::models::WeekScheduleResponse get_workforcemanagement_managementunit_week_schedule(management_unit_id, week_id, schedule_id, expand, force_download_service)
Deprecated.  Use the equivalent business unit resource instead. Get a week schedule

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**management_unit_id** | **String** | The ID of the management unit, or 'mine' for the management unit of the logged-in user. | [required] |
**week_id** | **String** | First day of schedule week in yyyy-MM-dd format. | [required] |
**schedule_id** | **String** | The ID of the schedule to fetch | [required] |
**expand** | Option<**String**> | Which fields, if any, to expand |  |
**force_download_service** | Option<**bool**> | Force the result of this operation to be sent via download service.  For testing/app development purposes |  |

### Return type

[**crate::models::WeekScheduleResponse**](WeekScheduleResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workforcemanagement_managementunit_week_schedules

> crate::models::WeekScheduleListResponse get_workforcemanagement_managementunit_week_schedules(management_unit_id, week_id, include_only_published, earliest_week_date, latest_week_date)
Deprecated.  Use the equivalent business unit resource instead. Get the list of schedules in a week in management unit

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**management_unit_id** | **String** | The ID of the management unit, or 'mine' for the management unit of the logged-in user. | [required] |
**week_id** | **String** | First day of schedule week in yyyy-MM-dd format. | [required] |
**include_only_published** | Option<**bool**> | Return only published schedules |  |
**earliest_week_date** | Option<**String**> | The start date of the earliest week to query in yyyy-MM-dd format |  |
**latest_week_date** | Option<**String**> | The start date of the latest week to query in yyyy-MM-dd format |  |

### Return type

[**crate::models::WeekScheduleListResponse**](WeekScheduleListResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workforcemanagement_managementunit_week_shifttrades

> crate::models::WeekShiftTradeListResponse get_workforcemanagement_managementunit_week_shifttrades(management_unit_id, week_date_id, evaluate_matches)
Gets all the shift trades for a given week

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**management_unit_id** | **String** | The management unit ID of the management unit, or 'mine' for the management unit of the logged-in user. | [required] |
**week_date_id** | **String** | The start date of the week schedule in yyyy-MM-dd format. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd | [required] |
**evaluate_matches** | Option<**bool**> | Whether to evaluate the matches for violations |  |[default to true]

### Return type

[**crate::models::WeekShiftTradeListResponse**](WeekShiftTradeListResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workforcemanagement_managementunit_workplan

> crate::models::WorkPlan get_workforcemanagement_managementunit_workplan(management_unit_id, work_plan_id, include_only)
Get a work plan

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**management_unit_id** | **String** | The ID of the management unit, or 'mine' for the management unit of the logged-in user. | [required] |
**work_plan_id** | **String** | The ID of the work plan to fetch | [required] |
**include_only** | Option<[**Vec<String>**](String.md)> | limit response to the specified fields |  |

### Return type

[**crate::models::WorkPlan**](WorkPlan.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workforcemanagement_managementunit_workplanrotation

> crate::models::WorkPlanRotationResponse get_workforcemanagement_managementunit_workplanrotation(management_unit_id, work_plan_rotation_id)
Get a work plan rotation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**management_unit_id** | **String** | The ID of the management unit, or 'mine' for the management unit of the logged-in user. | [required] |
**work_plan_rotation_id** | **String** | The ID of the work plan rotation to fetch | [required] |

### Return type

[**crate::models::WorkPlanRotationResponse**](WorkPlanRotationResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workforcemanagement_managementunit_workplanrotations

> crate::models::WorkPlanRotationListResponse get_workforcemanagement_managementunit_workplanrotations(management_unit_id, expand)
Get work plan rotations

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**management_unit_id** | **String** | The ID of the management unit, or 'mine' for the management unit of the logged-in user. | [required] |
**expand** | Option<[**Vec<String>**](String.md)> |  |  |

### Return type

[**crate::models::WorkPlanRotationListResponse**](WorkPlanRotationListResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workforcemanagement_managementunit_workplans

> crate::models::WorkPlanListResponse get_workforcemanagement_managementunit_workplans(management_unit_id, expand)
Get work plans

\"expand=details\" is deprecated

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**management_unit_id** | **String** | The ID of the management unit, or 'mine' for the management unit of the logged-in user. | [required] |
**expand** | Option<[**Vec<String>**](String.md)> |  |  |

### Return type

[**crate::models::WorkPlanListResponse**](WorkPlanListResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workforcemanagement_managementunits

> crate::models::ManagementUnitListing get_workforcemanagement_managementunits(page_size, page_number, expand, feature, division_id)
Get management units

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | Deprecated, paging is not supported |  |
**page_number** | Option<**i32**> | Deprecated, paging is not supported |  |
**expand** | Option<**String**> | Deprecated, expand settings on the single MU route |  |
**feature** | Option<**String**> |  |  |
**division_id** | Option<**String**> |  |  |

### Return type

[**crate::models::ManagementUnitListing**](ManagementUnitListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workforcemanagement_managementunits_divisionviews

> crate::models::ManagementUnitListing get_workforcemanagement_managementunits_divisionviews(division_id)
Get management units across divisions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**division_id** | Option<[**Vec<String>**](String.md)> | The divisionIds to filter by. If omitted, will return all divisions |  |

### Return type

[**crate::models::ManagementUnitListing**](ManagementUnitListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workforcemanagement_notifications

> crate::models::NotificationsResponse get_workforcemanagement_notifications()
Get a list of notifications for the current user

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::NotificationsResponse**](NotificationsResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workforcemanagement_schedulingjob

> crate::models::SchedulingStatusResponse get_workforcemanagement_schedulingjob(job_id)
Get status of the scheduling job

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | The id of the scheduling job | [required] |

### Return type

[**crate::models::SchedulingStatusResponse**](SchedulingStatusResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workforcemanagement_shifttrades

> crate::models::ShiftTradeListResponse get_workforcemanagement_shifttrades()
Gets all of my shift trades

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ShiftTradeListResponse**](ShiftTradeListResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workforcemanagement_timeoffrequest

> crate::models::TimeOffRequestResponse get_workforcemanagement_timeoffrequest(time_off_request_id)
Get a time off request for the current user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**time_off_request_id** | **String** | Time Off Request Id | [required] |

### Return type

[**crate::models::TimeOffRequestResponse**](TimeOffRequestResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workforcemanagement_timeoffrequest_waitlistpositions

> crate::models::WaitlistPositionListing get_workforcemanagement_timeoffrequest_waitlistpositions(time_off_request_id)
Get the daily waitlist positions of a time off request for the current user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**time_off_request_id** | **String** | Time Off Request Id | [required] |

### Return type

[**crate::models::WaitlistPositionListing**](WaitlistPositionListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workforcemanagement_timeoffrequests

> crate::models::TimeOffRequestList get_workforcemanagement_timeoffrequests(recently_reviewed)
Get a list of time off requests for the current user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**recently_reviewed** | Option<**bool**> | Limit results to requests that have been reviewed within the preceding 30 days |  |[default to false]

### Return type

[**crate::models::TimeOffRequestList**](TimeOffRequestList.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_workforcemanagement_businessunit

> crate::models::BusinessUnit patch_workforcemanagement_businessunit(business_unit_id, body)
Update business unit

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**business_unit_id** | **String** | The ID of the business unit, or 'mine' for the business unit of the logged-in user. | [required] |
**body** | Option<[**UpdateBusinessUnitRequest**](UpdateBusinessUnitRequest.md)> | body |  |

### Return type

[**crate::models::BusinessUnit**](BusinessUnit.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_workforcemanagement_businessunit_activitycode

> crate::models::BusinessUnitActivityCode patch_workforcemanagement_businessunit_activitycode(business_unit_id, activity_code_id, body)
Update an activity code

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**business_unit_id** | **String** | The ID of the business unit, or 'mine' for the business unit of the logged-in user. | [required] |
**activity_code_id** | **String** | The ID of the activity code to update | [required] |
**body** | Option<[**UpdateActivityCodeRequest**](UpdateActivityCodeRequest.md)> | body |  |

### Return type

[**crate::models::BusinessUnitActivityCode**](BusinessUnitActivityCode.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_workforcemanagement_businessunit_planninggroup

> crate::models::PlanningGroup patch_workforcemanagement_businessunit_planninggroup(business_unit_id, planning_group_id, body)
Updates the planning group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**business_unit_id** | **String** | The ID of the business unit. | [required] |
**planning_group_id** | **String** | The ID of a planning group to update | [required] |
**body** | Option<[**UpdatePlanningGroupRequest**](UpdatePlanningGroupRequest.md)> | body |  |

### Return type

[**crate::models::PlanningGroup**](PlanningGroup.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_workforcemanagement_businessunit_scheduling_run

> patch_workforcemanagement_businessunit_scheduling_run(business_unit_id, run_id, body)
Mark a schedule run as applied

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**business_unit_id** | **String** | The ID of the business unit | [required] |
**run_id** | **String** | The ID of the schedule run | [required] |
**body** | Option<[**PatchBuScheduleRunRequest**](PatchBuScheduleRunRequest.md)> | body |  |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_workforcemanagement_businessunit_servicegoaltemplate

> crate::models::ServiceGoalTemplate patch_workforcemanagement_businessunit_servicegoaltemplate(business_unit_id, service_goal_template_id, body)
Updates a service goal template

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**business_unit_id** | **String** | The ID of the business unit. | [required] |
**service_goal_template_id** | **String** | The ID of a service goal template to update | [required] |
**body** | Option<[**UpdateServiceGoalTemplate**](UpdateServiceGoalTemplate.md)> | body |  |

### Return type

[**crate::models::ServiceGoalTemplate**](ServiceGoalTemplate.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_workforcemanagement_managementunit

> crate::models::ManagementUnit patch_workforcemanagement_managementunit(management_unit_id, body)
Update the requested management unit

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**management_unit_id** | **String** | The ID of the management unit, or 'mine' for the management unit of the logged-in user. | [required] |
**body** | Option<[**UpdateManagementUnitRequest**](UpdateManagementUnitRequest.md)> | body |  |

### Return type

[**crate::models::ManagementUnit**](ManagementUnit.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_workforcemanagement_managementunit_timeofflimit

> crate::models::TimeOffLimit patch_workforcemanagement_managementunit_timeofflimit(management_unit_id, time_off_limit_id, body)
Updates a time off limit object.

Updates time off limit object properties, but not daily values.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**management_unit_id** | **String** | The management unit ID of the management unit. | [required] |
**time_off_limit_id** | **String** | The id of time off limit object to update | [required] |
**body** | Option<[**UpdateTimeOffLimitRequest**](UpdateTimeOffLimitRequest.md)> | body |  |

### Return type

[**crate::models::TimeOffLimit**](TimeOffLimit.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_workforcemanagement_managementunit_timeoffplan

> crate::models::TimeOffPlan patch_workforcemanagement_managementunit_timeoffplan(management_unit_id, time_off_plan_id, body)
Updates a time off plan

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**management_unit_id** | **String** | The management unit ID. | [required] |
**time_off_plan_id** | **String** | The ID of the time off plan to update | [required] |
**body** | Option<[**UpdateTimeOffPlanRequest**](UpdateTimeOffPlanRequest.md)> | body |  |

### Return type

[**crate::models::TimeOffPlan**](TimeOffPlan.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_workforcemanagement_managementunit_user_timeoffrequest

> crate::models::TimeOffRequestResponse patch_workforcemanagement_managementunit_user_timeoffrequest(management_unit_id, user_id, time_off_request_id, body)
Update a time off request

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**management_unit_id** | **String** | The muId of the management unit, or 'mine' for the management unit of the logged-in user. | [required] |
**user_id** | **String** | The id of the user the requested time off request belongs to | [required] |
**time_off_request_id** | **String** | The id of the time off request to update | [required] |
**body** | Option<[**AdminTimeOffRequestPatch**](AdminTimeOffRequestPatch.md)> | body |  |

### Return type

[**crate::models::TimeOffRequestResponse**](TimeOffRequestResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_workforcemanagement_managementunit_week_shifttrade

> crate::models::ShiftTradeResponse patch_workforcemanagement_managementunit_week_shifttrade(management_unit_id, week_date_id, trade_id, body)
Updates a shift trade. This route can only be called by the initiating agent

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**management_unit_id** | **String** | The management unit ID of the management unit, or 'mine' for the management unit of the logged-in user. | [required] |
**week_date_id** | **String** | The start date of the week schedule in yyyy-MM-dd format. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd | [required] |
**trade_id** | **String** | The ID of the shift trade to update | [required] |
**body** | [**PatchShiftTradeRequest**](PatchShiftTradeRequest.md) | body | [required] |

### Return type

[**crate::models::ShiftTradeResponse**](ShiftTradeResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_workforcemanagement_managementunit_workplan

> crate::models::WorkPlan patch_workforcemanagement_managementunit_workplan(management_unit_id, work_plan_id, validation_mode, body)
Update a work plan

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**management_unit_id** | **String** | The ID of the management unit, or 'mine' for the management unit of the logged-in user. | [required] |
**work_plan_id** | **String** | The ID of the work plan to update | [required] |
**validation_mode** | Option<**String**> | Allows to update work plan even if validation result is invalid |  |
**body** | Option<[**WorkPlan**](WorkPlan.md)> | body |  |

### Return type

[**crate::models::WorkPlan**](WorkPlan.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_workforcemanagement_managementunit_workplanrotation

> crate::models::WorkPlanRotationResponse patch_workforcemanagement_managementunit_workplanrotation(management_unit_id, work_plan_rotation_id, body)
Update a work plan rotation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**management_unit_id** | **String** | The ID of the management unit, or 'mine' for the management unit of the logged-in user. | [required] |
**work_plan_rotation_id** | **String** | The ID of the work plan rotation to update | [required] |
**body** | Option<[**UpdateWorkPlanRotationRequest**](UpdateWorkPlanRotationRequest.md)> | body |  |

### Return type

[**crate::models::WorkPlanRotationResponse**](WorkPlanRotationResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_workforcemanagement_timeoffrequest

> crate::models::TimeOffRequestResponse patch_workforcemanagement_timeoffrequest(time_off_request_id, body)
Update a time off request for the current user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**time_off_request_id** | **String** | Time Off Request Id | [required] |
**body** | Option<[**AgentTimeOffRequestPatch**](AgentTimeOffRequestPatch.md)> | body |  |

### Return type

[**crate::models::TimeOffRequestResponse**](TimeOffRequestResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_workforcemanagement_adherence_historical

> crate::models::WfmHistoricalAdherenceResponse post_workforcemanagement_adherence_historical(body)
Request a historical adherence report for users across management units

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**WfmHistoricalAdherenceQueryForUsers**](WfmHistoricalAdherenceQueryForUsers.md)> | body |  |

### Return type

[**crate::models::WfmHistoricalAdherenceResponse**](WfmHistoricalAdherenceResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_workforcemanagement_agentschedules_mine

> crate::models::BuCurrentAgentScheduleSearchResponse post_workforcemanagement_agentschedules_mine(body)
Get published schedule for the current user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**BuGetCurrentAgentScheduleRequest**](BuGetCurrentAgentScheduleRequest.md)> | body |  |

### Return type

[**crate::models::BuCurrentAgentScheduleSearchResponse**](BuCurrentAgentScheduleSearchResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_workforcemanagement_businessunit_activitycodes

> crate::models::BusinessUnitActivityCode post_workforcemanagement_businessunit_activitycodes(business_unit_id, body)
Create a new activity code

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**business_unit_id** | **String** | The ID of the business unit, or 'mine' for the business unit of the logged-in user. | [required] |
**body** | Option<[**CreateActivityCodeRequest**](CreateActivityCodeRequest.md)> | body |  |

### Return type

[**crate::models::BusinessUnitActivityCode**](BusinessUnitActivityCode.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_workforcemanagement_businessunit_agentschedules_search

> crate::models::BuAsyncAgentSchedulesSearchResponse post_workforcemanagement_businessunit_agentschedules_search(business_unit_id, force_async, force_download_service, body)
Search published schedules

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**business_unit_id** | **String** | The ID of the business unit | [required] |
**force_async** | Option<**bool**> | Force the result of this operation to be sent asynchronously via notification.  For testing/app development purposes |  |
**force_download_service** | Option<**bool**> | Force the result of this operation to be sent via download service.  For testing/app development purposes |  |
**body** | Option<[**BuSearchAgentSchedulesRequest**](BuSearchAgentSchedulesRequest.md)> | body |  |

### Return type

[**crate::models::BuAsyncAgentSchedulesSearchResponse**](BuAsyncAgentSchedulesSearchResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_workforcemanagement_businessunit_intraday

> crate::models::AsyncIntradayResponse post_workforcemanagement_businessunit_intraday(business_unit_id, force_async, body)
Get intraday data for the given date for the requested planningGroupIds

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**business_unit_id** | **String** | The business unit ID of the business unit | [required] |
**force_async** | Option<**bool**> | Force the result of this operation to be sent asynchronously via notification.  For testing/app development purposes |  |
**body** | Option<[**IntradayPlanningGroupRequest**](IntradayPlanningGroupRequest.md)> | body |  |

### Return type

[**crate::models::AsyncIntradayResponse**](AsyncIntradayResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_workforcemanagement_businessunit_planninggroups

> crate::models::PlanningGroup post_workforcemanagement_businessunit_planninggroups(business_unit_id, body)
Adds a new planning group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**business_unit_id** | **String** | The ID of the business unit. | [required] |
**body** | Option<[**CreatePlanningGroupRequest**](CreatePlanningGroupRequest.md)> | body |  |

### Return type

[**crate::models::PlanningGroup**](PlanningGroup.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_workforcemanagement_businessunit_servicegoaltemplates

> crate::models::ServiceGoalTemplate post_workforcemanagement_businessunit_servicegoaltemplates(business_unit_id, body)
Adds a new service goal template

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**business_unit_id** | **String** | The ID of the business unit. | [required] |
**body** | Option<[**CreateServiceGoalTemplate**](CreateServiceGoalTemplate.md)> | body |  |

### Return type

[**crate::models::ServiceGoalTemplate**](ServiceGoalTemplate.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_workforcemanagement_businessunit_week_schedule_agentschedules_query

> crate::models::BuAsyncAgentSchedulesQueryResponse post_workforcemanagement_businessunit_week_schedule_agentschedules_query(business_unit_id, week_id, schedule_id, body, force_async, force_download_service)
Loads agent schedule data from the schedule. Used in combination with the metadata route

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**business_unit_id** | **String** | The ID of the business unit | [required] |
**week_id** | **String** | First day of schedule week in yyyy-MM-dd format. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd | [required] |
**schedule_id** | **String** | The ID of the schedule | [required] |
**body** | [**BuQueryAgentSchedulesRequest**](BuQueryAgentSchedulesRequest.md) | body | [required] |
**force_async** | Option<**bool**> | Force the result of this operation to be sent asynchronously via notification.  For testing/app development purposes |  |
**force_download_service** | Option<**bool**> | Force the result of this operation to be sent via download service.  For testing/app development purposes |  |

### Return type

[**crate::models::BuAsyncAgentSchedulesQueryResponse**](BuAsyncAgentSchedulesQueryResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_workforcemanagement_businessunit_week_schedule_copy

> crate::models::BuAsyncScheduleResponse post_workforcemanagement_businessunit_week_schedule_copy(business_unit_id, week_id, schedule_id, body)
Copy a schedule

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**business_unit_id** | **String** | The ID of the business unit | [required] |
**week_id** | **String** | First day of schedule week in yyyy-MM-dd format. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd | [required] |
**schedule_id** | **String** | The ID of the schedule to copy | [required] |
**body** | [**BuCopyScheduleRequest**](BuCopyScheduleRequest.md) | body | [required] |

### Return type

[**crate::models::BuAsyncScheduleResponse**](BuAsyncScheduleResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_workforcemanagement_businessunit_week_schedule_reschedule

> crate::models::BuAsyncScheduleRunResponse post_workforcemanagement_businessunit_week_schedule_reschedule(business_unit_id, week_id, schedule_id, body)
Start a rescheduling run

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**business_unit_id** | **String** | The ID of the business unit | [required] |
**week_id** | **String** | First day of schedule week in yyyy-MM-dd format. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd | [required] |
**schedule_id** | **String** | The ID of the schedule | [required] |
**body** | [**BuRescheduleRequest**](BuRescheduleRequest.md) | body | [required] |

### Return type

[**crate::models::BuAsyncScheduleRunResponse**](BuAsyncScheduleRunResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_workforcemanagement_businessunit_week_schedules

> crate::models::BuScheduleMetadata post_workforcemanagement_businessunit_week_schedules(business_unit_id, week_id, body)
Create a blank schedule

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**business_unit_id** | **String** | The ID of the business unit | [required] |
**week_id** | **String** | First day of schedule week in yyyy-MM-dd format. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd | [required] |
**body** | [**BuCreateBlankScheduleRequest**](BuCreateBlankScheduleRequest.md) | body | [required] |

### Return type

[**crate::models::BuScheduleMetadata**](BuScheduleMetadata.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_workforcemanagement_businessunit_week_schedules_generate

> crate::models::BuAsyncScheduleRunResponse post_workforcemanagement_businessunit_week_schedules_generate(business_unit_id, week_id, body)
Generate a schedule

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**business_unit_id** | **String** | The ID of the business unit | [required] |
**week_id** | **String** | First day of schedule week in yyyy-MM-dd format. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd | [required] |
**body** | [**BuGenerateScheduleRequest**](BuGenerateScheduleRequest.md) | body | [required] |

### Return type

[**crate::models::BuAsyncScheduleRunResponse**](BuAsyncScheduleRunResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_workforcemanagement_businessunit_week_shorttermforecast_copy

> crate::models::AsyncForecastOperationResult post_workforcemanagement_businessunit_week_shorttermforecast_copy(business_unit_id, week_date_id, forecast_id, body, force_async)
Copy a short term forecast

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**business_unit_id** | **String** | The business unit ID of the business unit to which the forecast belongs | [required] |
**week_date_id** | **String** | The week start date of the forecast in yyyy-MM-dd format. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd | [required] |
**forecast_id** | **String** | The ID of the forecast to copy | [required] |
**body** | [**CopyBuForecastRequest**](CopyBuForecastRequest.md) | body | [required] |
**force_async** | Option<**bool**> | Force the result of this operation to be sent asynchronously via notification.  For testing/app development purposes |  |

### Return type

[**crate::models::AsyncForecastOperationResult**](AsyncForecastOperationResult.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_workforcemanagement_businessunit_week_shorttermforecasts_generate

> crate::models::AsyncForecastOperationResult post_workforcemanagement_businessunit_week_shorttermforecasts_generate(business_unit_id, week_date_id, body, force_async)
Generate a short term forecast

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**business_unit_id** | **String** | The business unit ID of the business unit to which the forecast belongs | [required] |
**week_date_id** | **String** | The week start date of the forecast in yyyy-MM-dd format. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd | [required] |
**body** | [**GenerateBuForecastRequest**](GenerateBuForecastRequest.md) |  | [required] |
**force_async** | Option<**bool**> | Force the result of this operation to be sent asynchronously via notification.  For testing/app development purposes |  |

### Return type

[**crate::models::AsyncForecastOperationResult**](AsyncForecastOperationResult.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_workforcemanagement_businessunits

> crate::models::BusinessUnit post_workforcemanagement_businessunits(body)
Add a new business unit

It may take a minute or two for a new business unit to be available for api operations

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**CreateBusinessUnitRequest**](CreateBusinessUnitRequest.md)> | body |  |

### Return type

[**crate::models::BusinessUnit**](BusinessUnit.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_workforcemanagement_calendar_url_ics

> crate::models::CalendarUrlResponse post_workforcemanagement_calendar_url_ics(language)
Create a newly generated calendar link for the current user; if the current user has previously generated one, the generated link will be returned

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**language** | Option<**String**> | A language tag (which is sometimes referred to as a \"locale identifier\") to use to localize default activity code names in the ics-formatted calendar |  |[default to en-US]

### Return type

[**crate::models::CalendarUrlResponse**](CalendarUrlResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_workforcemanagement_historicaldata_deletejob

> crate::models::HistoricalImportDeleteJobResponse post_workforcemanagement_historicaldata_deletejob()
Delete the entries of the historical data imports in the organization

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::HistoricalImportDeleteJobResponse**](HistoricalImportDeleteJobResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_workforcemanagement_historicaldata_validate

> post_workforcemanagement_historicaldata_validate(body)
Trigger validation process for historical import

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ValidationServiceRequest**](ValidationServiceRequest.md)> | body |  |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_workforcemanagement_managementunit_agentschedules_search

> crate::models::BuAsyncAgentSchedulesSearchResponse post_workforcemanagement_managementunit_agentschedules_search(management_unit_id, force_async, force_download_service, body)
Query published schedules for given given time range for set of users

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**management_unit_id** | **String** | The management unit ID of the management unit, or 'mine' for the management unit of the logged-in user. | [required] |
**force_async** | Option<**bool**> | Force the result of this operation to be sent asynchronously via notification.  For testing/app development purposes |  |
**force_download_service** | Option<**bool**> | Force the result of this operation to be sent via download service.  For testing/app development purposes |  |
**body** | Option<[**BuSearchAgentSchedulesRequest**](BuSearchAgentSchedulesRequest.md)> | body |  |

### Return type

[**crate::models::BuAsyncAgentSchedulesSearchResponse**](BuAsyncAgentSchedulesSearchResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_workforcemanagement_managementunit_historicaladherencequery

> crate::models::WfmHistoricalAdherenceResponse post_workforcemanagement_managementunit_historicaladherencequery(management_unit_id, body)
Request a historical adherence report

The maximum supported range for historical adherence queries is 31 days, or 7 days with includeExceptions = true

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**management_unit_id** | **String** | The management unit ID of the management unit | [required] |
**body** | Option<[**WfmHistoricalAdherenceQuery**](WfmHistoricalAdherenceQuery.md)> | body |  |

### Return type

[**crate::models::WfmHistoricalAdherenceResponse**](WfmHistoricalAdherenceResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_workforcemanagement_managementunit_move

> crate::models::MoveManagementUnitResponse post_workforcemanagement_managementunit_move(management_unit_id, body)
Move the requested management unit to a new business unit

Returns status 200 if the management unit is already in the requested business unit

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**management_unit_id** | **String** | The ID of the management unit, or 'mine' for the management unit of the logged-in user. | [required] |
**body** | Option<[**MoveManagementUnitRequest**](MoveManagementUnitRequest.md)> | body |  |

### Return type

[**crate::models::MoveManagementUnitResponse**](MoveManagementUnitResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_workforcemanagement_managementunit_schedules_search

> crate::models::UserScheduleContainer post_workforcemanagement_managementunit_schedules_search(management_unit_id, body)
Query published schedules for given given time range for set of users

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**management_unit_id** | **String** | The management unit ID of the management unit, or 'mine' for the management unit of the logged-in user. | [required] |
**body** | Option<[**UserListScheduleRequestBody**](UserListScheduleRequestBody.md)> | body |  |

### Return type

[**crate::models::UserScheduleContainer**](UserScheduleContainer.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_workforcemanagement_managementunit_timeofflimits

> crate::models::TimeOffLimit post_workforcemanagement_managementunit_timeofflimits(management_unit_id, body)
Creates a new time off limit object under management unit.

Only one limit object is allowed under management unit, so an attempt to create second object will fail.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**management_unit_id** | **String** | The management unit ID of the management unit. | [required] |
**body** | Option<[**CreateTimeOffLimitRequest**](CreateTimeOffLimitRequest.md)> | body |  |

### Return type

[**crate::models::TimeOffLimit**](TimeOffLimit.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_workforcemanagement_managementunit_timeofflimits_values_query

> crate::models::QueryTimeOffLimitValuesResponse post_workforcemanagement_managementunit_timeofflimits_values_query(management_unit_id, body)
Retrieves time off limit related values based on a given set of filters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**management_unit_id** | **String** | The management unit ID of the management unit. | [required] |
**body** | Option<[**QueryTimeOffLimitValuesRequest**](QueryTimeOffLimitValuesRequest.md)> | body |  |

### Return type

[**crate::models::QueryTimeOffLimitValuesResponse**](QueryTimeOffLimitValuesResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_workforcemanagement_managementunit_timeoffplans

> crate::models::TimeOffPlan post_workforcemanagement_managementunit_timeoffplans(management_unit_id, body)
Creates a new time off plan

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**management_unit_id** | **String** | The management unit ID. | [required] |
**body** | Option<[**CreateTimeOffPlanRequest**](CreateTimeOffPlanRequest.md)> | body |  |

### Return type

[**crate::models::TimeOffPlan**](TimeOffPlan.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_workforcemanagement_managementunit_timeoffrequests

> crate::models::TimeOffRequestList post_workforcemanagement_managementunit_timeoffrequests(management_unit_id, body)
Create a new time off request

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**management_unit_id** | **String** | The muId of the management unit, or 'mine' for the management unit of the logged-in user. | [required] |
**body** | Option<[**CreateAdminTimeOffRequest**](CreateAdminTimeOffRequest.md)> | body |  |

### Return type

[**crate::models::TimeOffRequestList**](TimeOffRequestList.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_workforcemanagement_managementunit_timeoffrequests_query

> crate::models::TimeOffRequestListing post_workforcemanagement_managementunit_timeoffrequests_query(management_unit_id, body)
Fetches time off requests matching the conditions specified in the request body

Request body requires one of the following: User ID is specified, statuses == [Pending] or date range to be specified and less than or equal to 33 days.  All other fields are filters

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**management_unit_id** | **String** | The muId of the management unit, or 'mine' for the management unit of the logged-in user. | [required] |
**body** | Option<[**TimeOffRequestQueryBody**](TimeOffRequestQueryBody.md)> | body |  |

### Return type

[**crate::models::TimeOffRequestListing**](TimeOffRequestListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_workforcemanagement_managementunit_timeoffrequests_waitlistpositions_query

> crate::models::WaitlistPositionListing post_workforcemanagement_managementunit_timeoffrequests_waitlistpositions_query(management_unit_id, body)
Retrieves daily waitlist position for a list of time off requests

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**management_unit_id** | **String** | The management unit ID of the management unit. | [required] |
**body** | Option<[**QueryWaitlistPositionsRequest**](QueryWaitlistPositionsRequest.md)> | body |  |

### Return type

[**crate::models::WaitlistPositionListing**](WaitlistPositionListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_workforcemanagement_managementunit_week_shifttrade_match

> crate::models::MatchShiftTradeResponse post_workforcemanagement_managementunit_week_shifttrade_match(management_unit_id, week_date_id, trade_id, body)
Matches a shift trade. This route can only be called by the receiving agent

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**management_unit_id** | **String** | The management unit ID of the management unit, or 'mine' for the management unit of the logged-in user. | [required] |
**week_date_id** | **String** | The start date of the week schedule in yyyy-MM-dd format. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd | [required] |
**trade_id** | **String** | The ID of the shift trade to update | [required] |
**body** | [**MatchShiftTradeRequest**](MatchShiftTradeRequest.md) | body | [required] |

### Return type

[**crate::models::MatchShiftTradeResponse**](MatchShiftTradeResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_workforcemanagement_managementunit_week_shifttrades

> crate::models::ShiftTradeResponse post_workforcemanagement_managementunit_week_shifttrades(management_unit_id, week_date_id, body)
Adds a shift trade

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**management_unit_id** | **String** | The management unit ID of the management unit, or 'mine' for the management unit of the logged-in user. | [required] |
**week_date_id** | **String** | The start date of the week schedule in yyyy-MM-dd format. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd | [required] |
**body** | [**AddShiftTradeRequest**](AddShiftTradeRequest.md) | body | [required] |

### Return type

[**crate::models::ShiftTradeResponse**](ShiftTradeResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_workforcemanagement_managementunit_week_shifttrades_search

> crate::models::SearchShiftTradesResponse post_workforcemanagement_managementunit_week_shifttrades_search(management_unit_id, week_date_id, body)
Searches for potential shift trade matches for the current agent

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**management_unit_id** | **String** | The management unit ID of the management unit, or 'mine' for the management unit of the logged-in user. | [required] |
**week_date_id** | **String** | The start date of the week schedule in yyyy-MM-dd format. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd | [required] |
**body** | [**SearchShiftTradesRequest**](SearchShiftTradesRequest.md) | body | [required] |

### Return type

[**crate::models::SearchShiftTradesResponse**](SearchShiftTradesResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_workforcemanagement_managementunit_week_shifttrades_state_bulk

> crate::models::BulkUpdateShiftTradeStateResponse post_workforcemanagement_managementunit_week_shifttrades_state_bulk(management_unit_id, week_date_id, body, force_async)
Updates the state of a batch of shift trades

Admin functionality is not supported with \"mine\".

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**management_unit_id** | **String** | The management unit ID of the management unit, or 'mine' for the management unit of the logged-in user. | [required] |
**week_date_id** | **String** | The start date of the week schedule in yyyy-MM-dd format. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd | [required] |
**body** | [**BulkShiftTradeStateUpdateRequest**](BulkShiftTradeStateUpdateRequest.md) | body | [required] |
**force_async** | Option<**bool**> | Force the result of this operation to be sent asynchronously via notification.  For testing/app development purposes |  |

### Return type

[**crate::models::BulkUpdateShiftTradeStateResponse**](BulkUpdateShiftTradeStateResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_workforcemanagement_managementunit_workplan_copy

> crate::models::WorkPlan post_workforcemanagement_managementunit_workplan_copy(management_unit_id, work_plan_id, body)
Create a copy of work plan

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**management_unit_id** | **String** | The ID of the management unit, or 'mine' for the management unit of the logged-in user. | [required] |
**work_plan_id** | **String** | The ID of the work plan to create a copy | [required] |
**body** | Option<[**CopyWorkPlan**](CopyWorkPlan.md)> | body |  |

### Return type

[**crate::models::WorkPlan**](WorkPlan.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_workforcemanagement_managementunit_workplan_validate

> crate::models::ValidateWorkPlanResponse post_workforcemanagement_managementunit_workplan_validate(management_unit_id, work_plan_id, expand, body)
Validate Work Plan

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**management_unit_id** | **String** | The ID of the management unit, or 'mine' for the management unit of the logged-in user. | [required] |
**work_plan_id** | **String** | The ID of the work plan to validate. For new work plan, use the word 'new' for the ID. | [required] |
**expand** | Option<[**Vec<String>**](String.md)> |  |  |
**body** | Option<[**WorkPlanValidationRequest**](WorkPlanValidationRequest.md)> | body |  |

### Return type

[**crate::models::ValidateWorkPlanResponse**](ValidateWorkPlanResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_workforcemanagement_managementunit_workplanrotation_copy

> crate::models::WorkPlanRotationResponse post_workforcemanagement_managementunit_workplanrotation_copy(management_unit_id, work_plan_rotation_id, body)
Create a copy of work plan rotation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**management_unit_id** | **String** | The ID of the management unit, or 'mine' for the management unit of the logged-in user. | [required] |
**work_plan_rotation_id** | **String** | The ID of the work plan rotation to create a copy | [required] |
**body** | Option<[**CopyWorkPlanRotationRequest**](CopyWorkPlanRotationRequest.md)> | body |  |

### Return type

[**crate::models::WorkPlanRotationResponse**](WorkPlanRotationResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_workforcemanagement_managementunit_workplanrotations

> crate::models::WorkPlanRotationResponse post_workforcemanagement_managementunit_workplanrotations(management_unit_id, body)
Create a new work plan rotation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**management_unit_id** | **String** | The ID of the management unit, or 'mine' for the management unit of the logged-in user. | [required] |
**body** | Option<[**AddWorkPlanRotationRequest**](AddWorkPlanRotationRequest.md)> | body |  |

### Return type

[**crate::models::WorkPlanRotationResponse**](WorkPlanRotationResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_workforcemanagement_managementunit_workplans

> crate::models::WorkPlan post_workforcemanagement_managementunit_workplans(management_unit_id, validation_mode, body)
Create a new work plan

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**management_unit_id** | **String** | The ID of the management unit, or 'mine' for the management unit of the logged-in user. | [required] |
**validation_mode** | Option<**String**> | Allows to create work plan even if the validation result is invalid |  |
**body** | Option<[**CreateWorkPlan**](CreateWorkPlan.md)> | body |  |

### Return type

[**crate::models::WorkPlan**](WorkPlan.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_workforcemanagement_managementunits

> crate::models::ManagementUnit post_workforcemanagement_managementunits(body)
Add a management unit

It may take a minute or two for a new management unit to be available for api operations

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**CreateManagementUnitApiRequest**](CreateManagementUnitApiRequest.md)> | body |  |

### Return type

[**crate::models::ManagementUnit**](ManagementUnit.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_workforcemanagement_notifications_update

> crate::models::UpdateNotificationsResponse post_workforcemanagement_notifications_update(body)
Mark a list of notifications as read or unread

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**UpdateNotificationsRequest**](UpdateNotificationsRequest.md)> | body |  |

### Return type

[**crate::models::UpdateNotificationsResponse**](UpdateNotificationsResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_workforcemanagement_schedules

> crate::models::UserScheduleContainer post_workforcemanagement_schedules(body)
Get published schedule for the current user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**CurrentUserScheduleRequestBody**](CurrentUserScheduleRequestBody.md)> | body |  |

### Return type

[**crate::models::UserScheduleContainer**](UserScheduleContainer.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_workforcemanagement_timeofflimits_available_query

> crate::models::AvailableTimeOffResponse post_workforcemanagement_timeofflimits_available_query(body)
Queries available time off for the current user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**AvailableTimeOffRequest**](AvailableTimeOffRequest.md)> | body |  |

### Return type

[**crate::models::AvailableTimeOffResponse**](AvailableTimeOffResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_workforcemanagement_timeoffrequests

> crate::models::TimeOffRequestResponse post_workforcemanagement_timeoffrequests(body)
Create a time off request for the current user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**CreateAgentTimeOffRequest**](CreateAgentTimeOffRequest.md)> | body |  |

### Return type

[**crate::models::TimeOffRequestResponse**](TimeOffRequestResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_workforcemanagement_managementunit_timeofflimit_values

> crate::models::TimeOffLimit put_workforcemanagement_managementunit_timeofflimit_values(management_unit_id, time_off_limit_id, body)
Sets daily values for a date range of time off limit object

Note that only limit daily values can be set through API, allocated and waitlisted values are read-only for time off limit API

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**management_unit_id** | **String** | The management unit ID of the management unit. | [required] |
**time_off_limit_id** | **String** | The ID of the time off limit object to set values for | [required] |
**body** | Option<[**SetTimeOffLimitValuesRequest**](SetTimeOffLimitValuesRequest.md)> | body |  |

### Return type

[**crate::models::TimeOffLimit**](TimeOffLimit.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

