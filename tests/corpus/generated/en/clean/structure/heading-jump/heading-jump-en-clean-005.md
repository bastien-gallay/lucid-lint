# Server Maintenance Guide

Follow these steps in order. Do not skip the backup.

## Before You Start

### Check Disk Space

Run the disk report. You need at least twenty gigabytes free.

### Notify Users

Send the maintenance notice one day ahead.

## During Maintenance

### Stop the Services

Stop the web service first, then the database.

#### Order Matters

Stopping the database first can corrupt open sessions.

### Apply Updates

Install the patches and reboot.

## After Maintenance

Confirm every service is green on the dashboard before closing the window.
