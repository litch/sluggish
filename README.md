# Sluggish

It's useful to have a node that sometimes just chills for awhile before accepting an HTLC.

This will do that.  Run this plugin, and then if the label of a paid invoice contains: `sluggish` or `keysend`, the plugin will sleep 30 seconds before finalizing the invoice.

That's it.  That's the plugin.

Goodnight folks.