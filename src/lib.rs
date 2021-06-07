//! # Compressible
//!
//! Compressible checks if a provided media type is compressible using compression
//! algorithms like brotli, gzip, deflate, etc.
//!
//! Dataset from https://github.com/jshttp/mime-db/blob/fa5e4ef3cc8907ec3c5ec5b85af0c63d7059a5cd/db.json
//!
//! ```
//! use compressible::is_compressible;
//!
//! assert_eq!(is_compressible("text/plain".to_string()), true);
//! ```
use mime::Mime;
use std::array::IntoIter;
use std::collections::HashMap;
use std::iter::FromIterator;

/// Returns `bool` indicating whether the provided content type is compressible
/// using compression algorithms like brotli, gzip, deflate, etc.
///
/// The provided content_type is parsed using https://docs.rs/mime/0.3.16/mime/
/// and returns `false` if the parsing fails.
pub fn is_compressible(content_type: String) -> bool {
    // Data obtained from https://github.com/jshttp/mime-db/blob/fa5e4ef3cc8907ec3c5ec5b85af0c63d7059a5cd/db.json
    let compressible_map = HashMap::<&str, bool>::from_iter(IntoIter::new([
        ("application/3gpdash-qoe-report+xml", true),
        ("application/3gpp-ims+xml", true),
        ("application/3gpphal+json", true),
        ("application/3gpphalforms+json", true),
        ("application/activity+json", true),
        ("application/alto-costmap+json", true),
        ("application/alto-costmapfilter+json", true),
        ("application/alto-directory+json", true),
        ("application/alto-endpointcost+json", true),
        ("application/alto-endpointcostparams+json", true),
        ("application/alto-endpointprop+json", true),
        ("application/alto-endpointpropparams+json", true),
        ("application/alto-error+json", true),
        ("application/alto-networkmap+json", true),
        ("application/alto-networkmapfilter+json", true),
        ("application/alto-updatestreamcontrol+json", true),
        ("application/alto-updatestreamparams+json", true),
        ("application/atom+xml", true),
        ("application/atomcat+xml", true),
        ("application/atomdeleted+xml", true),
        ("application/atomsvc+xml", true),
        ("application/atsc-dwd+xml", true),
        ("application/atsc-held+xml", true),
        ("application/atsc-rdt+json", true),
        ("application/atsc-rsat+xml", true),
        ("application/auth-policy+xml", true),
        ("application/beep+xml", true),
        ("application/calendar+json", true),
        ("application/calendar+xml", true),
        ("application/captive+json", true),
        ("application/ccmp+xml", true),
        ("application/ccxml+xml", true),
        ("application/cdfx+xml", true),
        ("application/cea-2018+xml", true),
        ("application/cellml+xml", true),
        ("application/clue+xml", true),
        ("application/clue_info+xml", true),
        ("application/cnrp+xml", true),
        ("application/coap-group+json", true),
        ("application/conference-info+xml", true),
        ("application/cpl+xml", true),
        ("application/csta+xml", true),
        ("application/cstadata+xml", true),
        ("application/csvm+json", true),
        ("application/dart", true),
        ("application/dash+xml", true),
        ("application/davmount+xml", true),
        ("application/dialog-info+xml", true),
        ("application/dicom+json", true),
        ("application/dicom+xml", true),
        ("application/dns+json", true),
        ("application/docbook+xml", true),
        ("application/dskpp+xml", true),
        ("application/dssc+xml", true),
        ("application/ecmascript", true),
        ("application/elm+json", true),
        ("application/elm+xml", true),
        ("application/emergencycalldata.cap+xml", true),
        ("application/emergencycalldata.comment+xml", true),
        ("application/emergencycalldata.control+xml", true),
        ("application/emergencycalldata.deviceinfo+xml", true),
        ("application/emergencycalldata.providerinfo+xml", true),
        ("application/emergencycalldata.serviceinfo+xml", true),
        ("application/emergencycalldata.subscriberinfo+xml", true),
        ("application/emergencycalldata.veds+xml", true),
        ("application/emma+xml", true),
        ("application/emotionml+xml", true),
        ("application/epp+xml", true),
        ("application/expect-ct-report+json", true),
        ("application/fdt+xml", true),
        ("application/fhir+json", true),
        ("application/fhir+xml", true),
        ("application/fido.trusted-apps+json", true),
        ("application/framework-attributes+xml", true),
        ("application/geo+json", true),
        ("application/geoxacml+xml", true),
        ("application/gml+xml", true),
        ("application/gpx+xml", true),
        ("application/held+xml", true),
        ("application/ibe-key-request+xml", true),
        ("application/ibe-pkg-reply+xml", true),
        ("application/im-iscomposing+xml", true),
        ("application/inkml+xml", true),
        ("application/its+xml", true),
        ("application/javascript", true),
        ("application/jf2feed+json", true),
        ("application/jose+json", true),
        ("application/jrd+json", true),
        ("application/jscalendar+json", true),
        ("application/json", true),
        ("application/json-patch+json", true),
        ("application/jsonml+json", true),
        ("application/jwk+json", true),
        ("application/jwk-set+json", true),
        ("application/kpml-request+xml", true),
        ("application/kpml-response+xml", true),
        ("application/ld+json", true),
        ("application/lgr+xml", true),
        ("application/load-control+xml", true),
        ("application/lost+xml", true),
        ("application/lostsync+xml", true),
        ("application/mads+xml", true),
        ("application/manifest+json", true),
        ("application/marcxml+xml", true),
        ("application/mathml+xml", true),
        ("application/mathml-content+xml", true),
        ("application/mathml-presentation+xml", true),
        (
            "application/mbms-associated-procedure-description+xml",
            true,
        ),
        ("application/mbms-deregister+xml", true),
        ("application/mbms-envelope+xml", true),
        ("application/mbms-msk+xml", true),
        ("application/mbms-msk-response+xml", true),
        ("application/mbms-protection-description+xml", true),
        ("application/mbms-reception-report+xml", true),
        ("application/mbms-register+xml", true),
        ("application/mbms-register-response+xml", true),
        ("application/mbms-schedule+xml", true),
        ("application/mbms-user-service-description+xml", true),
        ("application/media-policy-dataset+xml", true),
        ("application/media_control+xml", true),
        ("application/mediaservercontrol+xml", true),
        ("application/merge-patch+json", true),
        ("application/metalink+xml", true),
        ("application/metalink4+xml", true),
        ("application/mets+xml", true),
        ("application/mmt-aei+xml", true),
        ("application/mmt-usd+xml", true),
        ("application/mods+xml", true),
        ("application/mrb-consumer+xml", true),
        ("application/mrb-publish+xml", true),
        ("application/msc-ivr+xml", true),
        ("application/msc-mixer+xml", true),
        ("application/mud+json", true),
        ("application/nlsml+xml", true),
        ("application/odm+xml", true),
        ("application/oebps-package+xml", true),
        ("application/omdoc+xml", true),
        ("application/opc-nodeset+xml", true),
        ("application/p2p-overlay+xml", true),
        ("application/patch-ops-error+xml", true),
        ("application/pidf+xml", true),
        ("application/pidf-diff+xml", true),
        ("application/pls+xml", true),
        ("application/poc-settings+xml", true),
        ("application/postscript", true),
        ("application/ppsp-tracker+json", true),
        ("application/problem+json", true),
        ("application/problem+xml", true),
        ("application/provenance+xml", true),
        ("application/prs.xsf+xml", true),
        ("application/pskc+xml", true),
        ("application/pvd+json", true),
        ("application/raml+yaml", true),
        ("application/rdap+json", true),
        ("application/rdf+xml", true),
        ("application/reginfo+xml", true),
        ("application/reputon+json", true),
        ("application/resource-lists+xml", true),
        ("application/resource-lists-diff+xml", true),
        ("application/rfc+xml", true),
        ("application/rlmi+xml", true),
        ("application/rls-services+xml", true),
        ("application/route-apd+xml", true),
        ("application/route-s-tsid+xml", true),
        ("application/route-usd+xml", true),
        ("application/rsd+xml", true),
        ("application/rss+xml", true),
        ("application/rtf", true),
        ("application/samlassertion+xml", true),
        ("application/samlmetadata+xml", true),
        ("application/sarif+json", true),
        ("application/sarif-external-properties+json", true),
        ("application/sbml+xml", true),
        ("application/scaip+xml", true),
        ("application/scim+json", true),
        ("application/senml+json", true),
        ("application/senml+xml", true),
        ("application/senml-etch+json", true),
        ("application/sensml+json", true),
        ("application/sensml+xml", true),
        ("application/sep+xml", true),
        ("application/shf+xml", true),
        ("application/simple-filter+xml", true),
        ("application/smil+xml", true),
        ("application/soap+xml", true),
        ("application/sparql-results+xml", true),
        ("application/spirits-event+xml", true),
        ("application/srgs+xml", true),
        ("application/sru+xml", true),
        ("application/ssdl+xml", true),
        ("application/ssml+xml", true),
        ("application/stix+json", true),
        ("application/swid+xml", true),
        ("application/tar", true),
        ("application/taxii+json", true),
        ("application/td+json", true),
        ("application/tei+xml", true),
        ("application/thraud+xml", true),
        ("application/tlsrpt+json", true),
        ("application/toml", true),
        ("application/ttml+xml", true),
        ("application/urc-grpsheet+xml", true),
        ("application/urc-ressheet+xml", true),
        ("application/urc-targetdesc+xml", true),
        ("application/urc-uisocketdesc+xml", true),
        ("application/vcard+json", true),
        ("application/vcard+xml", true),
        ("application/vnd.1000minds.decision-model+xml", true),
        ("application/vnd.3gpp-prose+xml", true),
        ("application/vnd.3gpp-prose-pc3ch+xml", true),
        ("application/vnd.3gpp.access-transfer-events+xml", true),
        ("application/vnd.3gpp.bsf+xml", true),
        ("application/vnd.3gpp.gmop+xml", true),
        ("application/vnd.3gpp.mcdata-affiliation-command+xml", true),
        ("application/vnd.3gpp.mcdata-info+xml", true),
        ("application/vnd.3gpp.mcdata-service-config+xml", true),
        ("application/vnd.3gpp.mcdata-ue-config+xml", true),
        ("application/vnd.3gpp.mcdata-user-profile+xml", true),
        ("application/vnd.3gpp.mcptt-affiliation-command+xml", true),
        ("application/vnd.3gpp.mcptt-floor-request+xml", true),
        ("application/vnd.3gpp.mcptt-info+xml", true),
        ("application/vnd.3gpp.mcptt-location-info+xml", true),
        ("application/vnd.3gpp.mcptt-mbms-usage-info+xml", true),
        ("application/vnd.3gpp.mcptt-service-config+xml", true),
        ("application/vnd.3gpp.mcptt-signed+xml", true),
        ("application/vnd.3gpp.mcptt-ue-config+xml", true),
        ("application/vnd.3gpp.mcptt-ue-init-config+xml", true),
        ("application/vnd.3gpp.mcptt-user-profile+xml", true),
        ("application/vnd.3gpp.mcvideo-affiliation-command+xml", true),
        ("application/vnd.3gpp.mcvideo-affiliation-info+xml", true),
        ("application/vnd.3gpp.mcvideo-info+xml", true),
        ("application/vnd.3gpp.mcvideo-location-info+xml", true),
        ("application/vnd.3gpp.mcvideo-mbms-usage-info+xml", true),
        ("application/vnd.3gpp.mcvideo-service-config+xml", true),
        (
            "application/vnd.3gpp.mcvideo-transmission-request+xml",
            true,
        ),
        ("application/vnd.3gpp.mcvideo-ue-config+xml", true),
        ("application/vnd.3gpp.mcvideo-user-profile+xml", true),
        ("application/vnd.3gpp.mid-call+xml", true),
        ("application/vnd.3gpp.sms+xml", true),
        ("application/vnd.3gpp.srvcc-ext+xml", true),
        ("application/vnd.3gpp.srvcc-info+xml", true),
        ("application/vnd.3gpp.state-and-event-info+xml", true),
        ("application/vnd.3gpp.ussd+xml", true),
        ("application/vnd.3gpp2.bcmcsinfo+xml", true),
        ("application/vnd.adobe.xdp+xml", true),
        ("application/vnd.amadeus+json", true),
        ("application/vnd.amundsen.maze+xml", true),
        ("application/vnd.api+json", true),
        ("application/vnd.aplextor.warrp+json", true),
        ("application/vnd.apothekende.reservation+json", true),
        ("application/vnd.apple.installer+xml", true),
        ("application/vnd.artisan+json", true),
        ("application/vnd.avalon+json", true),
        ("application/vnd.avistar+xml", true),
        ("application/vnd.balsamiq.bmml+xml", true),
        ("application/vnd.bbf.usp.msg+json", true),
        ("application/vnd.bekitzur-stech+json", true),
        ("application/vnd.biopax.rdf+xml", true),
        ("application/vnd.byu.uapi+json", true),
        ("application/vnd.capasystems-pg+json", true),
        ("application/vnd.chemdraw+xml", true),
        ("application/vnd.citationstyles.style+xml", true),
        ("application/vnd.collection+json", true),
        ("application/vnd.collection.doc+json", true),
        ("application/vnd.collection.next+json", true),
        ("application/vnd.coreos.ignition+json", true),
        ("application/vnd.criticaltools.wbs+xml", true),
        ("application/vnd.cryptii.pipe+json", true),
        ("application/vnd.ctct.ws+xml", true),
        ("application/vnd.cyan.dean.root+xml", true),
        ("application/vnd.cyclonedx+json", true),
        ("application/vnd.cyclonedx+xml", true),
        ("application/vnd.dart", true),
        ("application/vnd.datapackage+json", true),
        ("application/vnd.dataresource+json", true),
        ("application/vnd.dece.ttml+xml", true),
        ("application/vnd.dm.delegation+xml", true),
        ("application/vnd.document+json", true),
        ("application/vnd.drive+json", true),
        ("application/vnd.dvb.dvbisl+xml", true),
        ("application/vnd.dvb.notif-aggregate-root+xml", true),
        ("application/vnd.dvb.notif-container+xml", true),
        ("application/vnd.dvb.notif-generic+xml", true),
        ("application/vnd.dvb.notif-ia-msglist+xml", true),
        (
            "application/vnd.dvb.notif-ia-registration-request+xml",
            true,
        ),
        (
            "application/vnd.dvb.notif-ia-registration-response+xml",
            true,
        ),
        ("application/vnd.dvb.notif-init+xml", true),
        ("application/vnd.emclient.accessrequest+xml", true),
        ("application/vnd.eprints.data+xml", true),
        ("application/vnd.eszigno3+xml", true),
        ("application/vnd.etsi.aoc+xml", true),
        ("application/vnd.etsi.cug+xml", true),
        ("application/vnd.etsi.iptvcommand+xml", true),
        ("application/vnd.etsi.iptvdiscovery+xml", true),
        ("application/vnd.etsi.iptvprofile+xml", true),
        ("application/vnd.etsi.iptvsad-bc+xml", true),
        ("application/vnd.etsi.iptvsad-cod+xml", true),
        ("application/vnd.etsi.iptvsad-npvr+xml", true),
        ("application/vnd.etsi.iptvservice+xml", true),
        ("application/vnd.etsi.iptvsync+xml", true),
        ("application/vnd.etsi.iptvueprofile+xml", true),
        ("application/vnd.etsi.mcid+xml", true),
        (
            "application/vnd.etsi.overload-control-policy-dataset+xml",
            true,
        ),
        ("application/vnd.etsi.pstn+xml", true),
        ("application/vnd.etsi.sci+xml", true),
        ("application/vnd.etsi.simservs+xml", true),
        ("application/vnd.etsi.tsl+xml", true),
        ("application/vnd.fujifilm.fb.jfi+xml", true),
        ("application/vnd.futoin+json", true),
        ("application/vnd.gentics.grd+json", true),
        ("application/vnd.geo+json", true),
        ("application/vnd.geocube+xml", true),
        ("application/vnd.google-earth.kml+xml", true),
        ("application/vnd.gov.sk.e-form+xml", true),
        ("application/vnd.gov.sk.xmldatacontainer+xml", true),
        ("application/vnd.hal+json", true),
        ("application/vnd.hal+xml", true),
        ("application/vnd.handheld-entertainment+xml", true),
        ("application/vnd.hc+json", true),
        ("application/vnd.heroku+json", true),
        ("application/vnd.hyper+json", true),
        ("application/vnd.hyper-item+json", true),
        ("application/vnd.hyperdrive+json", true),
        ("application/vnd.ims.lis.v2.result+json", true),
        ("application/vnd.ims.lti.v2.toolconsumerprofile+json", true),
        ("application/vnd.ims.lti.v2.toolproxy+json", true),
        ("application/vnd.ims.lti.v2.toolproxy.id+json", true),
        ("application/vnd.ims.lti.v2.toolsettings+json", true),
        ("application/vnd.ims.lti.v2.toolsettings.simple+json", true),
        ("application/vnd.informedcontrol.rms+xml", true),
        ("application/vnd.infotech.project+xml", true),
        ("application/vnd.iptc.g2.catalogitem+xml", true),
        ("application/vnd.iptc.g2.conceptitem+xml", true),
        ("application/vnd.iptc.g2.knowledgeitem+xml", true),
        ("application/vnd.iptc.g2.newsitem+xml", true),
        ("application/vnd.iptc.g2.newsmessage+xml", true),
        ("application/vnd.iptc.g2.packageitem+xml", true),
        ("application/vnd.iptc.g2.planningitem+xml", true),
        ("application/vnd.irepository.package+xml", true),
        ("application/vnd.las.las+json", true),
        ("application/vnd.las.las+xml", true),
        ("application/vnd.leap+json", true),
        ("application/vnd.liberty-request+xml", true),
        (
            "application/vnd.llamagraphics.life-balance.exchange+xml",
            true,
        ),
        ("application/vnd.marlin.drm.actiontoken+xml", true),
        ("application/vnd.marlin.drm.conftoken+xml", true),
        ("application/vnd.marlin.drm.license+xml", true),
        ("application/vnd.mason+json", true),
        ("application/vnd.micro+json", true),
        ("application/vnd.miele+json", true),
        ("application/vnd.mozilla.xul+xml", true),
        ("application/vnd.ms-fontobject", true),
        ("application/vnd.ms-office.activex+xml", true),
        ("application/vnd.ms-opentype", true),
        ("application/vnd.ms-playready.initiator+xml", true),
        ("application/vnd.ms-printdevicecapabilities+xml", true),
        ("application/vnd.ms-printing.printticket+xml", true),
        ("application/vnd.ms-printschematicket+xml", true),
        ("application/vnd.nearst.inv+json", true),
        ("application/vnd.nokia.conml+xml", true),
        ("application/vnd.nokia.iptv.config+xml", true),
        ("application/vnd.nokia.landmark+xml", true),
        ("application/vnd.nokia.landmarkcollection+xml", true),
        ("application/vnd.nokia.n-gage.ac+xml", true),
        ("application/vnd.nokia.pcd+xml", true),
        ("application/vnd.oci.image.manifest.v1+json", true),
        ("application/vnd.oftn.l10n+json", true),
        ("application/vnd.oipf.contentaccessdownload+xml", true),
        ("application/vnd.oipf.contentaccessstreaming+xml", true),
        ("application/vnd.oipf.dae.svg+xml", true),
        ("application/vnd.oipf.dae.xhtml+xml", true),
        ("application/vnd.oipf.mippvcontrolmessage+xml", true),
        ("application/vnd.oipf.spdiscovery+xml", true),
        ("application/vnd.oipf.spdlist+xml", true),
        ("application/vnd.oipf.ueprofile+xml", true),
        ("application/vnd.oipf.userprofile+xml", true),
        (
            "application/vnd.oma.bcast.associated-procedure-parameter+xml",
            true,
        ),
        ("application/vnd.oma.bcast.drm-trigger+xml", true),
        ("application/vnd.oma.bcast.imd+xml", true),
        ("application/vnd.oma.bcast.notification+xml", true),
        ("application/vnd.oma.bcast.sgdd+xml", true),
        ("application/vnd.oma.bcast.smartcard-trigger+xml", true),
        ("application/vnd.oma.bcast.sprov+xml", true),
        ("application/vnd.oma.cab-address-book+xml", true),
        ("application/vnd.oma.cab-feature-handler+xml", true),
        ("application/vnd.oma.cab-pcc+xml", true),
        ("application/vnd.oma.cab-subs-invite+xml", true),
        ("application/vnd.oma.cab-user-prefs+xml", true),
        ("application/vnd.oma.dd2+xml", true),
        ("application/vnd.oma.drm.risd+xml", true),
        ("application/vnd.oma.group-usage-list+xml", true),
        ("application/vnd.oma.lwm2m+json", true),
        ("application/vnd.oma.pal+xml", true),
        ("application/vnd.oma.poc.detailed-progress-report+xml", true),
        ("application/vnd.oma.poc.final-report+xml", true),
        ("application/vnd.oma.poc.groups+xml", true),
        ("application/vnd.oma.poc.invocation-descriptor+xml", true),
        (
            "application/vnd.oma.poc.optimized-progress-report+xml",
            true,
        ),
        ("application/vnd.oma.scidm.messages+xml", true),
        ("application/vnd.oma.xcap-directory+xml", true),
        ("application/vnd.omads-email+xml", true),
        ("application/vnd.omads-file+xml", true),
        ("application/vnd.omads-folder+xml", true),
        ("application/vnd.openblox.game+xml", true),
        ("application/vnd.openstreetmap.data+xml", true),
        (
            "application/vnd.openxmlformats-officedocument.custom-properties+xml",
            true,
        ),
        (
            "application/vnd.openxmlformats-officedocument.customxmlproperties+xml",
            true,
        ),
        (
            "application/vnd.openxmlformats-officedocument.drawing+xml",
            true,
        ),
        (
            "application/vnd.openxmlformats-officedocument.drawingml.chart+xml",
            true,
        ),
        (
            "application/vnd.openxmlformats-officedocument.drawingml.chartshapes+xml",
            true,
        ),
        (
            "application/vnd.openxmlformats-officedocument.drawingml.diagramcolors+xml",
            true,
        ),
        (
            "application/vnd.openxmlformats-officedocument.drawingml.diagramdata+xml",
            true,
        ),
        (
            "application/vnd.openxmlformats-officedocument.drawingml.diagramlayout+xml",
            true,
        ),
        (
            "application/vnd.openxmlformats-officedocument.drawingml.diagramstyle+xml",
            true,
        ),
        (
            "application/vnd.openxmlformats-officedocument.extended-properties+xml",
            true,
        ),
        (
            "application/vnd.openxmlformats-officedocument.presentationml.commentauthors+xml",
            true,
        ),
        (
            "application/vnd.openxmlformats-officedocument.presentationml.comments+xml",
            true,
        ),
        (
            "application/vnd.openxmlformats-officedocument.presentationml.handoutmaster+xml",
            true,
        ),
        (
            "application/vnd.openxmlformats-officedocument.presentationml.notesmaster+xml",
            true,
        ),
        (
            "application/vnd.openxmlformats-officedocument.presentationml.notesslide+xml",
            true,
        ),
        (
            "application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml",
            true,
        ),
        (
            "application/vnd.openxmlformats-officedocument.presentationml.presprops+xml",
            true,
        ),
        (
            "application/vnd.openxmlformats-officedocument.presentationml.slide+xml",
            true,
        ),
        (
            "application/vnd.openxmlformats-officedocument.presentationml.slidelayout+xml",
            true,
        ),
        (
            "application/vnd.openxmlformats-officedocument.presentationml.slidemaster+xml",
            true,
        ),
        (
            "application/vnd.openxmlformats-officedocument.presentationml.slideshow.main+xml",
            true,
        ),
        (
            "application/vnd.openxmlformats-officedocument.presentationml.slideupdateinfo+xml",
            true,
        ),
        (
            "application/vnd.openxmlformats-officedocument.presentationml.tablestyles+xml",
            true,
        ),
        (
            "application/vnd.openxmlformats-officedocument.presentationml.tags+xml",
            true,
        ),
        (
            "application/vnd.openxmlformats-officedocument.presentationml.template.main+xml",
            true,
        ),
        (
            "application/vnd.openxmlformats-officedocument.presentationml.viewprops+xml",
            true,
        ),
        (
            "application/vnd.openxmlformats-officedocument.spreadsheetml.calcchain+xml",
            true,
        ),
        (
            "application/vnd.openxmlformats-officedocument.spreadsheetml.chartsheet+xml",
            true,
        ),
        (
            "application/vnd.openxmlformats-officedocument.spreadsheetml.comments+xml",
            true,
        ),
        (
            "application/vnd.openxmlformats-officedocument.spreadsheetml.connections+xml",
            true,
        ),
        (
            "application/vnd.openxmlformats-officedocument.spreadsheetml.dialogsheet+xml",
            true,
        ),
        (
            "application/vnd.openxmlformats-officedocument.spreadsheetml.externallink+xml",
            true,
        ),
        (
            "application/vnd.openxmlformats-officedocument.spreadsheetml.pivotcachedefinition+xml",
            true,
        ),
        (
            "application/vnd.openxmlformats-officedocument.spreadsheetml.pivotcacherecords+xml",
            true,
        ),
        (
            "application/vnd.openxmlformats-officedocument.spreadsheetml.pivottable+xml",
            true,
        ),
        (
            "application/vnd.openxmlformats-officedocument.spreadsheetml.querytable+xml",
            true,
        ),
        (
            "application/vnd.openxmlformats-officedocument.spreadsheetml.revisionheaders+xml",
            true,
        ),
        (
            "application/vnd.openxmlformats-officedocument.spreadsheetml.revisionlog+xml",
            true,
        ),
        (
            "application/vnd.openxmlformats-officedocument.spreadsheetml.sharedstrings+xml",
            true,
        ),
        (
            "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml",
            true,
        ),
        (
            "application/vnd.openxmlformats-officedocument.spreadsheetml.sheetmetadata+xml",
            true,
        ),
        (
            "application/vnd.openxmlformats-officedocument.spreadsheetml.styles+xml",
            true,
        ),
        (
            "application/vnd.openxmlformats-officedocument.spreadsheetml.table+xml",
            true,
        ),
        (
            "application/vnd.openxmlformats-officedocument.spreadsheetml.tablesinglecells+xml",
            true,
        ),
        (
            "application/vnd.openxmlformats-officedocument.spreadsheetml.template.main+xml",
            true,
        ),
        (
            "application/vnd.openxmlformats-officedocument.spreadsheetml.usernames+xml",
            true,
        ),
        (
            "application/vnd.openxmlformats-officedocument.spreadsheetml.volatiledependencies+xml",
            true,
        ),
        (
            "application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml",
            true,
        ),
        (
            "application/vnd.openxmlformats-officedocument.theme+xml",
            true,
        ),
        (
            "application/vnd.openxmlformats-officedocument.themeoverride+xml",
            true,
        ),
        (
            "application/vnd.openxmlformats-officedocument.wordprocessingml.comments+xml",
            true,
        ),
        (
            "application/vnd.openxmlformats-officedocument.wordprocessingml.document.glossary+xml",
            true,
        ),
        (
            "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml",
            true,
        ),
        (
            "application/vnd.openxmlformats-officedocument.wordprocessingml.endnotes+xml",
            true,
        ),
        (
            "application/vnd.openxmlformats-officedocument.wordprocessingml.fonttable+xml",
            true,
        ),
        (
            "application/vnd.openxmlformats-officedocument.wordprocessingml.footer+xml",
            true,
        ),
        (
            "application/vnd.openxmlformats-officedocument.wordprocessingml.footnotes+xml",
            true,
        ),
        (
            "application/vnd.openxmlformats-officedocument.wordprocessingml.numbering+xml",
            true,
        ),
        (
            "application/vnd.openxmlformats-officedocument.wordprocessingml.settings+xml",
            true,
        ),
        (
            "application/vnd.openxmlformats-officedocument.wordprocessingml.styles+xml",
            true,
        ),
        (
            "application/vnd.openxmlformats-officedocument.wordprocessingml.template.main+xml",
            true,
        ),
        (
            "application/vnd.openxmlformats-officedocument.wordprocessingml.websettings+xml",
            true,
        ),
        (
            "application/vnd.openxmlformats-package.core-properties+xml",
            true,
        ),
        (
            "application/vnd.openxmlformats-package.digital-signature-xmlsignature+xml",
            true,
        ),
        (
            "application/vnd.openxmlformats-package.relationships+xml",
            true,
        ),
        ("application/vnd.oracle.resource+json", true),
        ("application/vnd.otps.ct-kip+xml", true),
        ("application/vnd.pagerduty+json", true),
        ("application/vnd.poc.group-advertisement+xml", true),
        ("application/vnd.pwg-xhtml-print+xml", true),
        ("application/vnd.radisys.moml+xml", true),
        ("application/vnd.radisys.msml+xml", true),
        ("application/vnd.radisys.msml-audit+xml", true),
        ("application/vnd.radisys.msml-audit-conf+xml", true),
        ("application/vnd.radisys.msml-audit-conn+xml", true),
        ("application/vnd.radisys.msml-audit-dialog+xml", true),
        ("application/vnd.radisys.msml-audit-stream+xml", true),
        ("application/vnd.radisys.msml-conf+xml", true),
        ("application/vnd.radisys.msml-dialog+xml", true),
        ("application/vnd.radisys.msml-dialog-base+xml", true),
        ("application/vnd.radisys.msml-dialog-fax-detect+xml", true),
        ("application/vnd.radisys.msml-dialog-fax-sendrecv+xml", true),
        ("application/vnd.radisys.msml-dialog-group+xml", true),
        ("application/vnd.radisys.msml-dialog-speech+xml", true),
        ("application/vnd.radisys.msml-dialog-transform+xml", true),
        ("application/vnd.recordare.musicxml+xml", true),
        ("application/vnd.restful+json", true),
        ("application/vnd.route66.link66+xml", true),
        ("application/vnd.seis+json", true),
        ("application/vnd.shootproof+json", true),
        ("application/vnd.shopkick+json", true),
        ("application/vnd.siren+json", true),
        ("application/vnd.software602.filler.form+xml", true),
        ("application/vnd.solent.sdkm+xml", true),
        ("application/vnd.sun.wadl+xml", true),
        ("application/vnd.sycle+xml", true),
        ("application/vnd.syncml+xml", true),
        ("application/vnd.syncml.dm+xml", true),
        ("application/vnd.syncml.dmddf+xml", true),
        ("application/vnd.syncml.dmtnds+xml", true),
        ("application/vnd.tableschema+json", true),
        ("application/vnd.think-cell.ppttc+json", true),
        ("application/vnd.tmd.mediaflex.api+xml", true),
        ("application/vnd.uoml+xml", true),
        ("application/vnd.vel+json", true),
        ("application/vnd.wv.csp+xml", true),
        ("application/vnd.wv.ssp+xml", true),
        ("application/vnd.xacml+json", true),
        ("application/vnd.xmi+xml", true),
        ("application/vnd.yamaha.openscoreformat.osfpvg+xml", true),
        ("application/vnd.zzazz.deck+xml", true),
        ("application/voicexml+xml", true),
        ("application/voucher-cms+json", true),
        ("application/wasm", true),
        ("application/watcherinfo+xml", true),
        ("application/webpush-options+json", true),
        ("application/wsdl+xml", true),
        ("application/wspolicy+xml", true),
        ("application/x-dtbncx+xml", true),
        ("application/x-dtbook+xml", true),
        ("application/x-dtbresource+xml", true),
        ("application/x-httpd-php", true),
        ("application/x-javascript", true),
        ("application/x-ns-proxy-autoconfig", true),
        ("application/x-sh", true),
        ("application/x-tar", true),
        ("application/x-virtualbox-hdd", true),
        ("application/x-virtualbox-ova", true),
        ("application/x-virtualbox-ovf", true),
        ("application/x-virtualbox-vbox", true),
        ("application/x-virtualbox-vdi", true),
        ("application/x-virtualbox-vhd", true),
        ("application/x-virtualbox-vmdk", true),
        ("application/x-web-app-manifest+json", true),
        ("application/x-www-form-urlencoded", true),
        ("application/x-xliff+xml", true),
        ("application/xacml+xml", true),
        ("application/xaml+xml", true),
        ("application/xcap-att+xml", true),
        ("application/xcap-caps+xml", true),
        ("application/xcap-diff+xml", true),
        ("application/xcap-el+xml", true),
        ("application/xcap-error+xml", true),
        ("application/xcap-ns+xml", true),
        ("application/xcon-conference-info+xml", true),
        ("application/xcon-conference-info-diff+xml", true),
        ("application/xenc+xml", true),
        ("application/xhtml+xml", true),
        ("application/xhtml-voice+xml", true),
        ("application/xliff+xml", true),
        ("application/xml", true),
        ("application/xml-dtd", true),
        ("application/xml-patch+xml", true),
        ("application/xmpp+xml", true),
        ("application/xop+xml", true),
        ("application/xproc+xml", true),
        ("application/xslt+xml", true),
        ("application/xspf+xml", true),
        ("application/xv+xml", true),
        ("application/yang-data+json", true),
        ("application/yang-data+xml", true),
        ("application/yang-patch+json", true),
        ("application/yang-patch+xml", true),
        ("application/yin+xml", true),
        ("font/otf", true),
        ("font/ttf", true),
        ("image/bmp", true),
        ("image/svg+xml", true),
        ("image/vnd.adobe.photoshop", true),
        ("image/x-icon", true),
        ("image/x-ms-bmp", true),
        ("message/imdn+xml", true),
        ("message/rfc822", true),
        ("model/gltf+json", true),
        ("model/gltf-binary", true),
        ("model/vnd.collada+xml", true),
        ("model/vnd.moml+xml", true),
        ("model/x3d+xml", true),
        ("text/cache-manifest", true),
        ("text/calender", true),
        ("text/cmd", true),
        ("text/css", true),
        ("text/csv", true),
        ("text/html", true),
        ("text/javascript", true),
        ("text/jsx", true),
        ("text/less", true),
        ("text/markdown", true),
        ("text/mdx", true),
        ("text/n3", true),
        ("text/plain", true),
        ("text/richtext", true),
        ("text/rtf", true),
        ("text/tab-separated-values", true),
        ("text/uri-list", true),
        ("text/vcard", true),
        ("text/vtt", true),
        ("text/x-gwt-rpc", true),
        ("text/x-jquery-tmpl", true),
        ("text/x-markdown", true),
        ("text/x-org", true),
        ("text/x-processing", true),
        ("text/x-suse-ymp", true),
        ("text/xml", true),
        ("text/yaml", true),
        ("x-shader/x-fragment", true),
        ("x-shader/x-vertex", true),
    ]));

    if let Ok(content_type) = content_type.parse::<Mime>() {
        if let Some(compressible) = compressible_map.get(content_type.essence_str()) {
            compressible.to_owned()
        } else {
            false
        }
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use crate::is_compressible;

    #[test]
    fn it_works() {
        assert_eq!(is_compressible("text/plain".to_string()), true);
        assert_eq!(
            is_compressible("application/x-web-app-manifest+json".to_string()),
            true
        );

        assert_eq!(is_compressible("image/jpeg; param=1".to_string()), false);
        assert_eq!(
            is_compressible("as;ldfkjas;ldfkja;lsdfj".to_string()),
            false
        );
    }
}
