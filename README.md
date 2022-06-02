# Portector
### A self contained port trap similar to PortSentry that handles connection attempts like Fail2Ban.

Currently under live test development. (I run this on my servers) Not recommended if you don't want a cluttered IPTables until I move the banned IPs to their own chain.

Supports logging to abuseipdb Bulk Report format.

## To-Do List
- [x] Define list of trap ports, make them configurable
- [x] Implement state saving
- [x] Implement hit count ban parameters
- [x] Implement whitelisting
- [ ] Handle OS signals for program termination (state save)
- [ ] Implement ban time
