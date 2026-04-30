param(
    [string]$TrackerOnePath = "$(Join-Path $PSScriptRoot '..\documents\Plans\Freelance\Client Outreach tracker 1.xlsx')",
    [string]$TrackerTwoPath = "$(Join-Path $PSScriptRoot '..\documents\Plans\Freelance\client outreach tracker 2.xlsx')",
    [string]$ProspectTargetsPath = "$(Join-Path $PSScriptRoot '..\documents\99_Agent_Ready\databases\prospect_targets.json')",
    [string]$OutputPath = "$(Join-Path $PSScriptRoot '..\documents\99_Agent_Ready\databases\independent_crm.json')"
)

$ErrorActionPreference = "Stop"

function Get-WorksheetRows {
    param(
        [Parameter(Mandatory = $true)]
        [string]$Path,
        [string]$SheetName = "sheet1.xml"
    )

    Add-Type -AssemblyName System.IO.Compression.FileSystem
    $zip = [IO.Compression.ZipFile]::OpenRead((Resolve-Path $Path))
    try {
        $shared = @()
        $sharedEntry = $zip.Entries | Where-Object { $_.FullName -eq "xl/sharedStrings.xml" }
        if ($sharedEntry) {
            $reader = [IO.StreamReader]::new($sharedEntry.Open())
            $sharedXml = [xml]$reader.ReadToEnd()
            $reader.Close()

            $stringItems = $sharedXml.GetElementsByTagName("si", "http://schemas.openxmlformats.org/spreadsheetml/2006/main")
            foreach ($item in $stringItems) {
                $textNodes = $item.GetElementsByTagName("t", "http://schemas.openxmlformats.org/spreadsheetml/2006/main")
                $parts = @()
                foreach ($textNode in $textNodes) {
                    $parts += $textNode.InnerText
                }
                $shared += ($parts -join "")
            }
        }

        $sheetEntry = $zip.Entries | Where-Object { $_.FullName -eq ("xl/worksheets/" + $SheetName) }
        if (-not $sheetEntry) {
            throw "Sheet $SheetName not found in $Path"
        }

        $sheetReader = [IO.StreamReader]::new($sheetEntry.Open())
        $sheetXml = [xml]$sheetReader.ReadToEnd()
        $sheetReader.Close()

        $rows = @()
        $rowNodes = $sheetXml.GetElementsByTagName("row", "http://schemas.openxmlformats.org/spreadsheetml/2006/main")
        foreach ($rowNode in $rowNodes) {
            $ordered = [ordered]@{}
            $cellNodes = $rowNode.GetElementsByTagName("c", "http://schemas.openxmlformats.org/spreadsheetml/2006/main")
            foreach ($cellNode in $cellNodes) {
                $ref = $cellNode.GetAttribute("r")
                $column = $ref -replace "\d", ""
                $cellType = $cellNode.GetAttribute("t")
                $valueNode = $cellNode.GetElementsByTagName("v", "http://schemas.openxmlformats.org/spreadsheetml/2006/main") | Select-Object -First 1
                $value = ""

                if ($cellType -eq "s" -and $valueNode) {
                    $value = [string]$shared[[int]$valueNode.InnerText]
                } elseif ($cellType -eq "inlineStr") {
                    $inlineNode = $cellNode.GetElementsByTagName("t", "http://schemas.openxmlformats.org/spreadsheetml/2006/main") | Select-Object -First 1
                    if ($inlineNode) {
                        $value = [string]$inlineNode.InnerText
                    }
                } elseif ($valueNode) {
                    $value = [string]$valueNode.InnerText
                }

                $ordered[$column] = $value.Trim()
            }

            $rows += [pscustomobject]$ordered
        }

        return $rows
    } finally {
        $zip.Dispose()
    }
}

function Normalize-OrganizationKey {
    param([string]$Value)

    if ([string]::IsNullOrWhiteSpace($Value)) {
        return ""
    }

    return (($Value.ToLowerInvariant() -replace "[^a-z0-9]+", "-").Trim("-"))
}

function Normalize-OptionalValue {
    param([AllowNull()][string]$Value)

    if ([string]::IsNullOrWhiteSpace($Value)) {
        return $null
    }

    $trimmed = $Value.Trim()
    if ($trimmed -in @("Not found", "not found", "N/A", "n/a")) {
        return $null
    }

    return $trimmed
}

function To-BoolOrNull {
    param([AllowNull()][string]$Value)

    $normalized = Normalize-OptionalValue $Value
    if ($null -eq $normalized) {
        return $null
    }

    switch ($normalized.ToLowerInvariant()) {
        "yes" { return $true }
        "no" { return $false }
        default { return $null }
    }
}

function Map-PriorityBand {
    param([AllowNull()][string]$Value)

    $normalized = Normalize-OptionalValue $Value
    $green = [string]([char]0xD83D) + [char]0xDFE2
    $yellow = [string]([char]0xD83D) + [char]0xDFE1
    $red = [string]([char]0xD83D) + [char]0xDD34

    if ($normalized -eq $green) {
        return "high"
    }
    if ($normalized -eq $yellow) {
        return "medium"
    }
    if ($normalized -eq $red) {
        return "low"
    }

    return "unknown"
}

function Add-Contact {
    param(
        [System.Collections.Generic.List[object]]$Contacts,
        [string]$Name,
        [string]$Role,
        [string]$Email,
        [string]$Source
    )

    $cleanName = Normalize-OptionalValue $Name
    $cleanRole = Normalize-OptionalValue $Role
    $cleanEmail = Normalize-OptionalValue $Email
    if ($null -eq $cleanName -and $null -eq $cleanRole -and $null -eq $cleanEmail) {
        return
    }

    foreach ($existing in $Contacts) {
        if ($existing.name -eq $cleanName -and $existing.role -eq $cleanRole -and $existing.email_candidate -eq $cleanEmail) {
            return
        }
    }

    $Contacts.Add([ordered]@{
        name = $cleanName
        role = $cleanRole
        email_candidate = $cleanEmail
        source = $Source
    })
}

$trackerOneRows = Get-WorksheetRows -Path $TrackerOnePath
$trackerTwoRows = Get-WorksheetRows -Path $TrackerTwoPath

$prospectTargets = @{}
if (Test-Path $ProspectTargetsPath) {
    $prospectPayload = Get-Content $ProspectTargetsPath -Raw | ConvertFrom-Json
    foreach ($target in $prospectPayload.targets) {
        $prospectTargets[(Normalize-OrganizationKey $target.organization)] = $target
    }
}

$byOrganization = [ordered]@{}

foreach ($row in ($trackerOneRows | Select-Object -Skip 1)) {
    $organization = Normalize-OptionalValue $row.A
    if ($null -eq $organization) {
        continue
    }

    $key = Normalize-OrganizationKey $organization
    $entry = [ordered]@{
        organization_id = $key
        organization = $organization
        country = Normalize-OptionalValue $row.B
        ownership_classification = "independent_candidate"
        pipeline_stage = "candidate"
        outreach_readiness = "research_more"
        priority_band = Map-PriorityBand $row.N
        focus_area = Normalize-OptionalValue $row.H
        fit_notes = Normalize-OptionalValue $row.M
        dataset_evidence_found = To-BoolOrNull $row.O
        recent_report_published = To-BoolOrNull $row.P
        recommended_language = $null
        recommended_entry_offer = $null
        contact_routes = [ordered]@{
            general_email_candidate = Normalize-OptionalValue $row.E
            organization_linkedin = Normalize-OptionalValue $row.F
            website = Normalize-OptionalValue $row.G
        }
        contacts = [System.Collections.Generic.List[object]]::new()
        outreach_personalization = [ordered]@{
            data_insights = $null
            suggested_message = $null
        }
        verification = [ordered]@{
            source_normalized = $true
            contact_verification_required = $true
            human_verified = $false
            ownership_verification_required = $true
            notes = @(
                "Normalized from workbook trackers; still requires human verification before any external send."
            )
        }
        source_refs = @(
            "documents/Plans/Freelance/Client Outreach tracker 1.xlsx"
        )
    }

    Add-Contact -Contacts $entry.contacts -Name $row.C -Role $row.D -Email $row.E -Source "tracker1_primary"
    $byOrganization[$key] = $entry
}

$roleColumnMap = [ordered]@{
    Q = "monitoring_evaluation_officer"
    R = "research_coordinator"
    S = "field_project_manager"
    T = "data_manager"
    U = "program_manager"
}

foreach ($row in ($trackerTwoRows | Select-Object -Skip 1)) {
    $organization = Normalize-OptionalValue $row.A
    if ($null -eq $organization) {
        continue
    }

    $key = Normalize-OrganizationKey $organization
    if (-not $byOrganization.Contains($key)) {
        $byOrganization[$key] = [ordered]@{
            organization_id = $key
            organization = $organization
            country = Normalize-OptionalValue $row.B
            ownership_classification = "independent_candidate"
            pipeline_stage = "candidate"
            outreach_readiness = "research_more"
            priority_band = Map-PriorityBand $row.N
            focus_area = Normalize-OptionalValue $row.H
            fit_notes = Normalize-OptionalValue $row.M
            dataset_evidence_found = To-BoolOrNull $row.O
            recent_report_published = To-BoolOrNull $row.P
            recommended_language = $null
            recommended_entry_offer = $null
            contact_routes = [ordered]@{
                general_email_candidate = Normalize-OptionalValue $row.E
                organization_linkedin = Normalize-OptionalValue $row.F
                website = Normalize-OptionalValue $row.G
            }
            contacts = [System.Collections.Generic.List[object]]::new()
            outreach_personalization = [ordered]@{
                data_insights = $null
                suggested_message = $null
            }
            verification = [ordered]@{
                source_normalized = $true
                contact_verification_required = $true
                human_verified = $false
                ownership_verification_required = $true
                notes = @(
                "Normalized from workbook trackers; still requires human verification before any external send."
            )
        }
        source_refs = @(
            "documents/Plans/Freelance/client outreach tracker 2.xlsx"
        )
        }
    }

    $entry = $byOrganization[$key]
    if ($null -eq $entry.country) { $entry.country = Normalize-OptionalValue $row.B }
    if ($null -eq $entry.focus_area) { $entry.focus_area = Normalize-OptionalValue $row.H }
    if ($entry.priority_band -eq "unknown") { $entry.priority_band = Map-PriorityBand $row.N }
    if ($null -eq $entry.fit_notes) { $entry.fit_notes = Normalize-OptionalValue $row.M }
    if ($null -eq $entry.dataset_evidence_found) { $entry.dataset_evidence_found = To-BoolOrNull $row.O }
    if ($null -eq $entry.recent_report_published) { $entry.recent_report_published = To-BoolOrNull $row.P }

    if ($null -eq $entry.contact_routes.general_email_candidate) {
        $entry.contact_routes.general_email_candidate = Normalize-OptionalValue $row.E
    }
    if ($null -eq $entry.contact_routes.organization_linkedin) {
        $entry.contact_routes.organization_linkedin = Normalize-OptionalValue $row.F
    }
    if ($null -eq $entry.contact_routes.website) {
        $entry.contact_routes.website = Normalize-OptionalValue $row.G
    }

    Add-Contact -Contacts $entry.contacts -Name $row.C -Role $row.D -Email $row.E -Source "tracker2_primary"
    foreach ($column in $roleColumnMap.Keys) {
        Add-Contact -Contacts $entry.contacts -Name $row.$column -Role $roleColumnMap[$column] -Email $null -Source "tracker2_role_column"
    }

    $entry.outreach_personalization.data_insights = Normalize-OptionalValue $row.V
    $entry.outreach_personalization.suggested_message = Normalize-OptionalValue $row.W

    if (-not ($entry.source_refs -contains "documents/Plans/Freelance/client outreach tracker 2.xlsx")) {
        $entry.source_refs += "documents/Plans/Freelance/client outreach tracker 2.xlsx"
    }
}

$normalizedLeads = New-Object System.Collections.Generic.List[object]
foreach ($key in $byOrganization.Keys) {
    $entry = $byOrganization[$key]
    $legacyTarget = $prospectTargets[$key]

    if ($legacyTarget) {
        $entry.recommended_language = Normalize-OptionalValue $legacyTarget.recommended_language
        $entry.recommended_entry_offer = Normalize-OptionalValue $legacyTarget.priority_offer
    } else {
        if ($entry.country -eq "Madagascar") {
            $entry.recommended_language = "english_or_french_needs_human_choice"
        } else {
            $entry.recommended_language = "english"
        }
        $entry.recommended_entry_offer = "Free 5-20 row sample review"
    }

    $hasReachableRoute = $null -ne $entry.contact_routes.general_email_candidate -or
        $null -ne $entry.contact_routes.organization_linkedin -or
        $null -ne $entry.contact_routes.website

    if ($entry.priority_band -eq "high" -and $hasReachableRoute) {
        $entry.outreach_readiness = "ready_for_human_review"
    } elseif ($entry.priority_band -eq "medium" -and $hasReachableRoute) {
        $entry.outreach_readiness = "secondary_queue"
    } else {
        $entry.outreach_readiness = "research_more"
    }

    $segment = if ($null -ne $entry.focus_area) {
        $entry.focus_area
    } else {
        "general-biodiversity-data-support"
    }
    $entry.segment = $segment
    $entry.funnel_tracking = [ordered]@{
        lead_id = $entry.organization_id
        ownership_classification = $entry.ownership_classification
        language = $entry.recommended_language
        segment = $segment
        offer_used = $null
        proof_asset_used = $null
        current_stage = $entry.pipeline_stage
        last_touch_at = $null
        next_action_at = $null
        outcome = $null
        human_verified = $entry.verification.human_verified
    }

    $entry.verification.notes += "No autonomous send. Human review and contact verification stay mandatory."
    $normalizedLeads.Add($entry)
}

$sortedLeads = $normalizedLeads | Sort-Object @{
    Expression = {
        switch ($_.priority_band) {
            "high" { 0 }
            "medium" { 1 }
            "low" { 2 }
            default { 3 }
        }
    }
}, @{
    Expression = { $_.country }
}, @{
    Expression = { $_.organization }
}

$priorityQueue = $sortedLeads |
    Where-Object { $_.outreach_readiness -eq "ready_for_human_review" } |
    Select-Object -First 15 |
    ForEach-Object { $_.organization_id }

$outputPayload = [ordered]@{
    generated_at = (Get-Date).ToString("yyyy-MM-dd")
    purpose = "Normalized independent, non-Techni CRM built from freelance outreach trackers."
    ownership_rule = "All entries remain independent_candidate leads until a human confirms they are outside Techni-Drones ownership and approves outreach."
    sources = @(
        "documents/Plans/Freelance/Client Outreach tracker 1.xlsx",
        "documents/Plans/Freelance/client outreach tracker 2.xlsx",
        "documents/99_Agent_Ready/databases/prospect_targets.json"
    )
    summary = [ordered]@{
        total_organizations = @($sortedLeads).Count
        ready_for_human_review = @($sortedLeads | Where-Object { $_.outreach_readiness -eq "ready_for_human_review" }).Count
        secondary_queue = @($sortedLeads | Where-Object { $_.outreach_readiness -eq "secondary_queue" }).Count
        research_more = @($sortedLeads | Where-Object { $_.outreach_readiness -eq "research_more" }).Count
        with_email_candidate = @($sortedLeads | Where-Object { $null -ne $_.contact_routes.general_email_candidate }).Count
        high_priority = @($sortedLeads | Where-Object { $_.priority_band -eq "high" }).Count
    }
    priority_queue = $priorityQueue
    leads = $sortedLeads
}

$outputDirectory = Split-Path -Parent $OutputPath
New-Item -ItemType Directory -Force -Path $outputDirectory | Out-Null

$json = $outputPayload | ConvertTo-Json -Depth 8
Set-Content -Path $OutputPath -Value $json -Encoding UTF8
Write-Host "Wrote normalized CRM to $OutputPath"
