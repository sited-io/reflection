job "reflection" {
  datacenters = ["dc1"]
  type        = "service"

  group "reflection-api" {
    count = 1

    network {
      mode = "bridge"

      port "grpc" {}
    }

    service {
      name = "reflection-api"
      port = "grpc"

      connect {
        sidecar_service {}
      }

      check {
        type     = "grpc"
        interval = "20s"
        timeout  = "2s"
      }
    }

    task "reflection-api" {
      driver = "docker"

      resources {
        cpu        = 100
        memory     = 256
        memory_max = 256
      }

      vault {
        policies = ["service-reflection"]
      }

      template {
        destination = "${NOMAD_SECRETS_DIR}/.env"
        env         = true
        change_mode = "restart"
        data        = <<EOF
{{ with nomadVar "nomad/jobs/reflection" }}
RUST_LOG='{{ .RUST_LOG }}'
{{ end }}

HOST='0.0.0.0:{{ env "NOMAD_PORT_grpc" }}'
EOF
      }

      config {
        image      = "__IMAGE__"
        force_pull = true
      }
    }
  }
}
