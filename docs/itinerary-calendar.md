# Put selected sessions in your calendar

Use the [day guide](../exports/day-guide.md) to compare the published program.
These calendars contain tentative choices from the three fictional
[participant paths](../community/paths/README.md):

- [Foundations calendar](../exports/calendars/foundations.ics)
- [Backend calendar](../exports/calendars/backend.ics)
- [Embedded calendar](../exports/calendars/embedded.ics)

Download one `.ics` file and import it into a separate calendar so you can remove
the example easily. Review times, tickets, workshop setup, and travel before
using its choices. This is a static snapshot; importing it does not subscribe
you to schedule changes. Calendar-app import behavior has not been tested.

Times are stored in UTC; your calendar displays them in its selected timezone.
The conference uses Europe/Madrid, UTC+02:00 on October 14–17, 2026. Venue names
come from the catalog; stage labels do not establish physical room names.
Check the [venue and entrance guide](time-and-place.md) and the organizer.

## Export your own choices

If you use the [optional helper](quickstart.md#install-a-reusable-binary), give it
an itinerary from the same snapshot. For a checked public example:

```sh
bin/conference-data export-itinerary-calendar \
  --itinerary community/paths/backend-itinerary.json \
  --output .plan/backend-sessions.ics
```

For your own plan, replace the itinerary path with your local JSON. The command
checks session IDs, availability, overlaps, and known end times before replacing
the output. Warnings still need your attention. Failed validation preserves an
existing output file.

Only selected sessions are exported; alternatives and conditional workshops are
excluded unless selected. Entries include public source links, retrieval dates,
and the snapshot ID. Profile names, goals, bookings, and selection reasons are
omitted. Chosen session titles can still reveal your interests if you share the file.
Entries are tentative and do not reserve a seat or record attendance.

Rebuild after a reviewed schedule change. Stable session IDs support comparisons,
but duplicate handling depends on your calendar app. The separate
[event and deadline calendar](../resources/stay-current/events.ics) covers future
conference dates and calls; it does not contain your EuroRust session choices.
